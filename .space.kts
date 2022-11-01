/**
* JetBrains Space Automation
* This Kotlin-script file lets you automate build activities
* For more info, see https://www.jetbrains.com/help/space/automation.html
*/

job("Hello World!") {
    container(displayName = "Say Hello", image = "ubuntu") {
    	shellScript {
            content = """
            	sudo apt install cargo -y
			    cargo build
            """
        }
    }
}
