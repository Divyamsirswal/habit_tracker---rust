use std::collections::HashMap;
use std::io;

/*
    Enhanced Habit Tracker CLI:
        -> Add habits.
        -> Mark habits as completed for the day.
        -> View progress over time.

        i think i will use hashmap to store habit {habit,boolean}
*/

fn main() {
    println!("\n========== Welcome to Personal Habit Tracker ==========");
    println!("Track your habits and achieve success one step at a time!\n");

    let mut habits: HashMap<String, bool> = HashMap::new();

    loop {
        println!("======================= Menu ==========================");
        println!("1. 📝 Add a new habit");
        println!("2. 📊 View your progress");
        println!("3. ✅ Mark a habit as completed");
        println!("4. ❌ Exit the tracker");
        println!("=======================================================");
        println!("👉 Please enter your choice:");

        let mut ch = String::new();
        io::stdin().read_line(&mut ch).expect("Failed to read line");
        let ch = ch.trim();

        match ch {
            "1" => {
                println!("\n📝 Enter the name of the habit you want to track:");
                let mut habit_name = String::new();
                io::stdin()
                    .read_line(&mut habit_name)
                    .expect("Failed to read the line");
                let habit_name = habit_name.trim().to_string();

                if habits.contains_key(&habit_name) {
                    println!("⚠️ Habit '{}' already exists!", habit_name);
                } else {
                    habits.insert(habit_name.clone(), false);
                    println!("✅ Habit '{}' has been added successfully!", habit_name);
                }
                println!();
            }
            "2" => {
                println!("\n📊 Your current habits and progress so far:");
                if habits.is_empty() {
                    println!("⚠️ No habits added yet. Start by adding one!");
                } else {
                    println!("-------------------------------------------------------");
                    for (habit, ok) in &habits {
                        let status = if *ok {
                            "✔️ Completed"
                        } else {
                            "❌ Pending"
                        };
                        println!(" - {}: {}", habit, status);
                    }
                    println!("-------------------------------------------------------");
                }
                println!();
            }
            "3" => {
                println!("\n✅ Enter the name of the habit you want to mark as completed:");
                let mut habit_name = String::new();
                io::stdin()
                    .read_line(&mut habit_name)
                    .expect("Failed to read line");
                let habit_name = habit_name.trim();

                if let Some(e) = habits.get_mut(habit_name) {
                    *e = true;
                    println!("🎉 Habit '{}' has been marked as completed!", habit_name);
                } else {
                    println!("⚠️ Habit '{}' not found. Please add it first.", habit_name);
                }
                println!();
            }
            "4" => {
                println!("\n👋 Exiting the tracker. Thanks for using it!");
                println!("Remember: Small steps lead to big achievements. 🌟");
                break;
            }
            _ => {
                println!("⚠️ Invalid choice! Please select a valid option from the menu.\n");
            }
        }
    }
}
