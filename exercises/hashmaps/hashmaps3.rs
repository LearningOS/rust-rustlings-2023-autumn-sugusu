// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;

// A structure to store the goal details of a team.
// 用于存储球队进球情况的结构体
struct Team {
    goals_scored: u8,      // 进球数
    goals_conceded: u8,    // 失球数
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    // 球队的名称作为键，与之相关的结构体作为值。
    let mut scores: HashMap<String, Team> = HashMap::new();  // 创建一个空的HashMap用于存储得分数据

    for r in results.lines() {  // 遍历输入字符串中的每一行
        let v: Vec<&str> = r.split(',').collect();  // 使用逗号分隔每行数据，并将结果存储为字符串切片的向量
        let team_1_name = v[0].to_string();  // 获取第一个球队的名称
        let team_1_score: u8 = v[2].parse().unwrap();  // 获取第一个球队的进球数并将其解析为无符号8位整数
        let team_2_name = v[1].to_string();  // 获取第二个球队的名称
        let team_2_score: u8 = v[3].parse().unwrap();  // 获取第二个球队的进球数并将其解析为无符号8位整数

        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        // 填充得分表，使用当前行提取的信息。要记住，由team_1进球数将等于team_2失球数，反之亦然。

        // Populate scores for team 1
        // 为第一个球队填充得分数据
        let team_1 = scores.entry(team_1_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });// 使用球队名称作为键，如果键不存在则插入一个新的Team结构体，否则返回已存在的Team结构体
        team_1.goals_scored += team_1_score;// 增加进球数
        team_1.goals_conceded += team_2_score;// 增加失球数

        // Populate scores for team 2
        // 为第二个球队填充得分数据
        let team_2 = scores.entry(team_2_name.clone()).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }
    scores  // 返回填充好的HashMap
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}