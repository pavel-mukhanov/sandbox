# Path to your oh-my-zsh installation.
export ZSH=/Users/pavelmukhanov/.oh-my-zsh

git-current-branch () {
    if ! git rev-parse 2> /dev/null
    then
        print "$0: not a repository: $PWD" >&2
        return 1
    fi
    local ref="$(git symbolic-ref HEAD 2> /dev/null)"
    if [[ -n "$ref" ]]
    then
        print "${ref#refs/heads/}"
        return 0
    else
        return 1
    fi
}

# Set name of the theme to load.
# Look in ~/.oh-my-zsh/themes/
# Optionally, if you set this to "random", it'll load a random theme each
# time that oh-my-zsh is loaded.
ZSH_THEME="honukai"

# PLUGINS
plugins=(git common-aliases)

# User configuration
export EDITOR="/usr/bin/vim"
export PATH=/Users/pavelmukhanov/Library/Android/sdk/tools:/Users/pavelmukhanov/Library/Android/sdk/build-tools/24.0.2:/Users/pavelmukhanov/Library/Android/sdk/platform-tools:/Users/pavelmukhanov/bin:/Applications/Genymotion.app/Contents/MacOS/:$PATH

source $ZSH/oh-my-zsh.sh

# You may need to manually set your language environment
export LANG=en_US.UTF-8

export PROJECT_HOME="/Users/pavelmukhanov/Android/TruckLoads"
export START_ACTIVITY=".view.tabs.TLTabActivity"
alias lg="git log --graph --abbrev-commit --decorate --format=format:'%C(bold blue)%h%C(reset) - %C(bold green)(%ar)%C(reset) %C(white)%s%C(reset) %C(dim white)- %an%C(reset)%C(bold yellow)%d%C(reset)'"
alias andr="cd $PROJECT_HOME"
alias clean="andr && ./gradlew clean"
alias install_debug="andr && ./gradlew installDevDebug --offline && adb shell am start com.truckerpath.truckloads/$START_ACTIVITY"
alias clean_install="clean && install_debug"
alias debug="adb shell am set-debug-app --persistent -w com.truckerpath.truckloads "
alias nodebug="adb shell am clear-debug-app com.truckerpath.truckloads"
alias reconnect="adb kill-server && adb connect 192.168.56.101"
alias killtl="adb shell ps | grep com.truckerpath.truckloads | cut -d' ' -f4| xargs adb shell kill -9"
alias pf='g push origin "$(git-current-branch 2> /dev/null)" --force'
alias td='f() {./gradlew testDevDebugUnitTest --tests "*.$1*" };andr && f'
alias debug_layout_on='adb shell setprop debug.layout true'
alias debug_layout_off='adb shell setprop debug.layout false'
alias gs='git status'
alias wipe_local_branches='git fetch -p && git branch -vv | awk " /: gone]/ {print \$1}" | xargs git branch -d'
alias gm4='gmtool admin start "Custom Phone - 4.1.1 - API 16 - 768x1280"'
alias gm5='gmtool admin start "Custom Phone - 5.0.0 - API 21 - 768x1280"'
alias gm6='gmtool admin start "Custom Phone - 6.0.0 - API 23 - 768x1280"'
alias settings='adb shell am start -a android.settings.APPLICATION_DETAILS_SETTINGS -d package:com.truckerpath.truckloads'

test -e "${HOME}/.iterm2_shell_integration.zsh" && source "${HOME}/.iterm2_shell_integration.zsh"
