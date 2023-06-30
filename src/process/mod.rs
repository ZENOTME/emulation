use futures::stream::BoxStream;

use self::end_process::EndProcess;
use self::error::Result;
use self::sub_process::SubProcess;
use crate::exchange::message::Message;

pub mod end_process;
pub mod error;
pub mod sub_process;

pub type BoxedMessageStream = BoxStream<'static, Result<Message>>;
pub type BoxedSubProcess = Box<dyn SubProcess>;
pub type BoxedEndProcess = Box<dyn EndProcess>;
