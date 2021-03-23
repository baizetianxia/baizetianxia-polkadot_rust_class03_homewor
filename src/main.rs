use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream:TcpStream) -> Result<(), Error> {
    let mut buf =[1;512];
    for _ in 0..100 {
        //从buf缓冲区读取客户端发来的数据
        let bytes_read = stream.read(&mut buf)?;
        //buf内数据大小为0，直接返回空结果
        if bytes_read == 0 {
            return Ok(());
        }
        //等待两秒
        thread::sleep(time::Duration::from_secs(2 as u64));
        //将buf缓冲区通过stream socket原样返回
        stream.write(&buf[..bytes_read])?;

        thread::sleep(time::Duration::from_secs(1 as u64))
    }
    Ok(())
}

fn main() ->std::io::Result<()> {
    //println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:8088")?;
    let mut thread_vec:Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming(){
        let stream = stream.expect("failed!");
        let handle = thread::spawn(move || {
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}",error));
        });

        thread_vec.push(handle);

    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())

}
