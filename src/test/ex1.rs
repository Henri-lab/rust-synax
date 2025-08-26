// 测试特性：闭包(Closures)、所有权转移(Ownership Transfer)、函数式编程
// 语法要点：impl Fn trait、闭包捕获、Vec操作、所有权移动
// 功能：演示如何使用闭包作为函数参数，处理任务列表并转移所有权
pub fn process_tasks(tasks: Vec<String>, handler: impl Fn(&String) -> bool) -> Vec<String> {
    let mut completed_tasks = Vec::new();

    for task in tasks {
        // 在这里调用闭包，判断任务是否处理成功
        // 如果成功，将任务的所有权移动到 completed_tasks 列表中
        if handler(&task) {
            completed_tasks.push(task);
        }
    }

    completed_tasks
}

pub fn main() {
    let my_tasks = vec![
        String::from("编写单元测试"),
        String::from("修复 Bug"),
        String::from("部署服务"),
        String::from("撰写文档"),
    ];

    // 定义一个闭包来处理任务。
    // 如果任务字符串包含“测试”或“部署”，则认为处理成功。
    let my_handler = |task: &String| -> bool {
        // 在这里写闭包的逻辑
        // 尝试使用 .contains() 方法检查子字符串
        task.contains("测试") || task.contains("部署")
    };

    // 调用 process_tasks 函数，并传入任务列表和闭包
    let completed_list = process_tasks(my_tasks, my_handler);

    // 打印已完成的任务列表
    println!("已完成的任务:");
    for task in completed_list {
        println!("- {}", task);
    }

    // 尝试在此处使用 my_tasks。你会发现它已经无法访问了。
    // println!("原始任务: {:?}", my_tasks); // 编译错误！
}