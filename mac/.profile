PS1=['\[\033[0;32m\]\u\[\033[0;32m\]@\h\[\033[00m\] \w \[\e[1;31m\]$(__git_ps1 "(%s)")\[\e[m\]]\$ '

HISTSIZE=1000000

export CLICOLOR=1
export LS_COLORS='Gxfxcxdxbxegedabagacad'
export EDITOR=vim

alias ls='ls -G'
alias ll='grc ls -laG'
alias grep='grep --color=auto'
alias db='mysql -u root -p test'
alias hs='history -a ; history -n'
alias fj='ps aux | grep jetty'
alias kj='kill -9'
alias clc='printf "\33[2J"'
alias mci='mvn clean install'
alias mcig='mvn clean install -DcreateGWT'
alias jrw='mvn jetty:run-war'
alias less='less -R'
alias mount_imob-dev="sshfs svn@imfe.jnpe.ru:/home/svn/site /Users/hz/imob-dev -p 22 -oauto_cache,reconnect,volname=imob-dev,follow_symlinks"
alias gs='git status '
alias ga='git add '
alias gb='git branch '
alias gc='git commit'
alias gd='git diff'
alias go='git checkout '
alias gk='gitk --all&'
alias gx='gitx --all'
alias mvn3='/usr/local/Cellar/maven/3.0.4/bin/mvn'
alias v='vim'
alias crontab="VIM_CRONTAB=true crontab"
alias resadb='adb kill-server && adb start-server'
alias todp='cp ~/XCodeProjects/Watee/android/target/classes/Watee.apk ~/Dropbox/Mukhanov/'

export LESSOPEN='| /usr/local/bin/src-hilite-lesspipe.sh %s'
#export HISTIGNORE="&:[ ]*:exit"
#export M2_HOME=/usr/share/maven
export GWT_HOME=/Users/hz/work/tools/gwt-2.0.2
export ANDROID_HOME=/Users/hz/android-sdk-mac_x86
export NDK_HOME=/Users/hz/android-ndk-r7b
export ANDROID_NDK=$NDK_HOME
export ANDROID_NDK_TOOLCHAIN_ROOT=/Users/hz/work/tools/android-toolchain
export ANDTOOLCHAIN=/Users/hz/work/tools/android-toolchain/android.toolchain.cmake
export PATH=~/bin:${M2_HOME}/bin:${ANDROID_HOME}/tools:${ANDROID_HOME}/platform-tools:${NDK_HOME}:${PATH}
# MacPorts Installer addition on 2011-02-08_at_14:47:52: adding an appropriate PATH variable for use with MacPorts.
export PATH=/usr/local/mysql/bin/:/usr/local/sbin:/Users/hz/android-ndk-r5b:/opt/local/bin:/usr/local/git/bin:/opt/local/sbin:/usr/local/mysql-5.5.9-osx10.6-x86_64/bin:~/work/scripts/:$PATH
# Finished adapting your PATH environment variable for use with MacPorts.
export LANG=ru_RU.UTF-8
export MAVEN_OPTS="-Xmx768m -XX:MaxPermSize=768m"

##
# Your previous /Users/hz/.profile file was backed up as /Users/hz/.profile.macports-saved_2011-10-26_at_04:57:50
##

# MacPorts Installer addition on 2011-10-26_at_04:57:50: adding an appropriate PATH variable for use with MacPorts.
export PATH=/opt/local/bin:/opt/local/sbin:$PATH
# Finished adapting your PATH environment variable for use with MacPorts.

export CLASSPATH=~/IdeaProjects/Study/lib/stdlib.jar:~/IdeaProjects/Study/algs4.jar:$CLASSPATH

trap ". ${HOME}/.profile" SIGUSR1

# Setting PATH for Python 2.7
# The orginal version is saved in .profile.pysave
PATH="/Library/Frameworks/Python.framework/Versions/2.7/bin:${PATH}"
export PATH
