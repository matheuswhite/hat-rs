*** Settings ***
# Boilerplate
Suite Setup       Setup
Suite Teardown    Teardown
Test Setup        Reset Emulation
Resource          ${RENODEKEYWORDS}

*** Keywords ***
Start Test
    [Arguments]    ${bin}
    # Create the Machine
    Execute Command         mach create
    # Load the stm32f413 board definitions
    Execute Command         machine LoadPlatformDescription @${CURDIR}/stm32f413.repl
    # Load the ELF file
    Execute Command         sysbus LoadELF @${CURDIR}/${bin}
    # Connect the UART
    Create Terminal Tester  sysbus.uart4
    Start Emulation

*** Test Cases ***
HelloWorld
    [Documentation]         Prints "Hello, World!" String
    [Tags]                  basic

    Start Test              helloworld

    Wait For Line On Uart   "Hello, World!"   timeout=2
