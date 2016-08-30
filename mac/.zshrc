# Path to your oh-my-zsh installation.
export ZSH=/Users/pavelmukhanov/.oh-my-zsh

# Set name of the theme to load.
# Look in ~/.oh-my-zsh/themes/
# Optionally, if you set this to "random", it'll load a random theme each
# time that oh-my-zsh is loaded.
ZSH_THEME="honukai"

# PLUGINS
plugins=(git)

# User configuration

export PATH=/Users/pavelmukhanov/Library/Android/sdk/tools:/Users/pavelmukhanov/Library/Android/sdk/platform-tools:$PATH

source $ZSH/oh-my-zsh.sh

# You may need to manually set your language environment
export LANG=en_US.UTF-8

export PROJECT_HOME="/Users/pavelmukhanov/Android/TruckLoads"
alias lg="git log --graph --abbrev-commit --decorate --format=format:'%C(bold blue)%h%C(reset) - %C(bold green)(%ar)%C(reset) %C(white)%s%C(reset) %C(dim white)- %an%C(reset)%C(bold yellow)%d%C(reset)'"
alias andr="cd $PROJECT_HOME"
alias install_debug="cd $PROJECT_HOME && ./gradlew installDevDebug --offline && adb shell am start com.truckerpath.truckloads/.view.splash.SplashActivity"

