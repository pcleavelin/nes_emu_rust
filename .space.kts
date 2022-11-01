/**
* JetBrains Space Automation
* This Kotlin-script file lets you automate build activities
* For more info, see https://www.jetbrains.com/help/space/automation.html
*/

job("Hello World!") {
    container(displayName = "Say Hello", image = "ubuntu") {
    	shellScript {
            content = """
            	curl https://sh.rustup.rs -sSf | bash -s -- -y
				echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
                
			    cargo build
            """
        }
    }
}
