mod advanced;
mod basic;
use crate::device::Device;
pub use advanced::AdvancedRemove;
pub use basic::BasicRemote;

// 这里与我们往常的写法不同，当我们去操作一个device的时候，我们
// 期望的是有可变的对象，那么这个trait是为了设计一个可变的手柄。
// 因为对于自己的本身，device改变在自身的时候，里面的函数直接self就好了
pub trait HasMutableDevice<D: Device> {
    fn device(&mut self) -> &mut D;
}

// 所以这里的Remote是遥控，必须要有拿到可变device的特征
pub trait Remote<D: Device>: HasMutableDevice<D> {
    fn new(device: D) -> Self;
    fn power(&mut self) {
        println!("Remote: power toggle");
        if self.device().is_enabled() {
            self.device().disable();
        } else {
            self.device().enable();
        }
    }
    fn volume_down(&mut self) {
        println!("Remote: volume down");
        let volume = self.device().volume();
        self.device().set_volume(volume - 10);
    }

    fn volume_up(&mut self) {
        println!("Remote: volume up");
        let volume = self.device().volume();
        self.device().set_volume(volume + 10);
    }

    fn channel_down(&mut self) {
        println!("Remote: channel down");
        let channel = self.device().channel();
        self.device().set_channel(channel - 1);
    }

    fn channel_up(&mut self) {
        println!("Remote: channel up");
        let channel = self.device().channel();
        self.device().set_channel(channel + 1);
    }
}
