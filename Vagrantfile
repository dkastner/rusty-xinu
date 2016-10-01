# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure(2) do |config|

  config.vm.define "default" do |default|
    default.vm.box = "debian/jessie64"

     default.vm.provision "shell", inline: <<-SHELL
        sudo apt-get update
        sudo apt-get install nasm -y
        sudo apt-get install xorriso -y
        sudo apt-get install git -y
        sudo apt-get install vim -y
        sudo apt-get install -y qemu
        curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh -s -- --yes multirust default nightly-2015-11-19 
    SHELL

    default.ssh.forward_x11 = true
  end

  config.vm.define "rusty-xinu" do |web|
    web.vm.box = "apache"
  end
end
