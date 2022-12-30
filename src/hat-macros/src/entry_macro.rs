use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{ExprParen, ExprTuple, Item, ItemFn};

pub fn entry_parse(args: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> Ast {
    if !args.is_empty() {
        panic!("Entry option cannot have an argument");
    }

    match syn::parse2::<Item>(item) {
        Ok(Item::Fn(item)) => item,
        Ok(_item) => {
            panic!("Item is not a function");
        }
        Err(_) => unreachable!()
    }
}

pub fn entry_analyze(ast: Ast) -> Model {
    let mut tasks = vec![];
    let mut channels = vec![];
    let mut triggers = vec![];

    let mut item = ast;

    let attrs = &mut item.attrs;
    for index in (0..attrs.len()).rev() {
        if attrs[index].path.segments.len() >= 2 {
            let ident = &attrs[index].path.segments[1].ident;

            match ident.to_string().as_str() {
                "tasks" => {
                    let attr = attrs.remove(index);

                    if let Ok(tuple) = syn::parse2::<ExprTuple>(attr.clone().tokens) {
                        for elem in tuple.elems {
                            if let Ok(ident) = syn::parse2::<Ident>(elem.to_token_stream()) {
                                tasks.push(ident);
                            }
                        }
                    } else if let Ok(expr) = syn::parse2::<ExprParen>(attr.clone().tokens) {
                        if let Ok(ident) = syn::parse2::<Ident>(expr.expr.to_token_stream()) {
                            tasks.push(ident);
                        }
                    }
                },
                "triggers" => {
                    let attr = attrs.remove(index);

                    if let Ok(tuple) = syn::parse2::<ExprTuple>(attr.clone().tokens) {
                        for elem in tuple.elems {
                            if let Ok(ident) = syn::parse2::<Ident>(elem.to_token_stream()) {
                                triggers.push(ident);
                            }
                        }
                    } else if let Ok(expr) = syn::parse2::<ExprParen>(attr.clone().tokens) {
                        if let Ok(ident) = syn::parse2::<Ident>(expr.expr.to_token_stream()) {
                            triggers.push(ident);
                        }
                    }
                },
                "channels" => {
                    let attr = attrs.remove(index);

                    if let Ok(tuple) = syn::parse2::<ExprTuple>(attr.clone().tokens) {
                        for elem in tuple.elems {
                            if let Ok(ident) = syn::parse2::<Ident>(elem.to_token_stream()) {
                                channels.push(ident);
                            }
                        }
                    } else if let Ok(expr) = syn::parse2::<ExprParen>(attr.clone().tokens) {
                        if let Ok(ident) = syn::parse2::<Ident>(expr.expr.to_token_stream()) {
                            channels.push(ident);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Model {
        tasks,
        channels,
        triggers,
        mutexes: vec![],
        semaphores: vec![],
        item,
    }
}

pub fn entry_codegen(mut model: Model) -> TokenStream {
    let old_name = model.item.sig.ident.clone();
    let new_name = format!("{}_inner", &old_name);
    model.item.sig.ident = Ident::new(&new_name, proc_macro2::Span::call_site());
    let new_name = model.item.sig.ident.clone();

    let mut channels = quote! {};
    for chan in model.channels {
        channels.append_all(quote! {
            #chan.init(Channel::new());
        });
    }

    let mut triggers = quote! {};
    for trig in model.triggers {
        triggers.append_all(quote! {
            #trig.init(Trigger::new());
        });
    }

    let mut tasks = quote! {};
    for (index, task) in model.tasks.iter().enumerate() {
        let task_name_upper = Ident::new(&task.to_string().to_uppercase(), proc_macro2::Span::call_site());
        let index = Literal::usize_suffixed(index + 1);

        tasks.append_all(quote! {
            static #task_name_upper: Lazy<Task> = Lazy::new();
            #task_name_upper.init(Task::new(#index, #task()));
        });
    }
    let mut tasks_list = quote! {};
    for task in &model.tasks {
        let task_name_upper = Ident::new(&task.to_string().to_uppercase(), proc_macro2::Span::call_site());

        tasks_list.append_all(quote! {
            , #task_name_upper.data()
        });
    }
    let tasks_amount = model.tasks.len() + 1;
    let model_item = model.item;

    let code = quote! {
        #[no_mangle]
        pub extern "C" fn #old_name() {
            timer_init();

            #triggers

            #channels

            #tasks

            static MAIN_TASK: Lazy<Task> = Lazy::new();
            MAIN_TASK.init(Task::new(0, #new_name()));

            static EXECUTOR: Lazy<Executor<#tasks_amount>> = Lazy::new();
            let vector = heapless::Vec::from_slice(&[MAIN_TASK.data() #tasks_list]).hat_expect("Cannot create task vector from slice");
            EXECUTOR.init(Executor::new(vector));

            let executor = EXECUTOR.data();
            match executor.run() {
                Ok(_) => log!("End of Executor"),
                Err(_) => log!("Executor error"),
            }
        }

        #model_item
    };
    code.into()
}

pub type Ast = ItemFn;

pub struct Model {
    pub tasks: Vec<Ident>,
    pub channels: Vec<Ident>,
    pub mutexes: Vec<Ident>,
    pub semaphores: Vec<Ident>,
    pub triggers: Vec<Ident>,
    pub item: ItemFn,
}
