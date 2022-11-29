fn solve(input: Vec<&str>) -> String {
    let mut remaining_players = input.clone();
    let mut remaining_players_index: Vec<usize> = (0..remaining_players.len()).collect();

    while remaining_players.len() > 1 {
        let mut limit = remaining_players.len() - 1;
        let mut current_index = 0;
        while current_index <= limit {
            if current_index == limit {
                remaining_players.remove(0);
                remaining_players_index.remove(0);
            } else {
                remaining_players.remove(current_index + 1);
                remaining_players_index.remove(current_index + 1);
            }
            limit = remaining_players.len() - 1;
            current_index += 1;
        }
    }
    return String::from(format!(
        "{}-{}",
        remaining_players[0], remaining_players_index[0]
    ));
}

fn main() {
    let input = Vec::from([
        "Gorusuke",
        "DavidFabian",
        "ItziarZG",
        "edy WOLF",
        "MarcosGaPe",
        "Jose Jimenez",
        "Borja ",
        "Jhonathan Izquierdo Higuita",
        "MiLfeR322",
        "Sebastián Espínola",
        "Matias Luna",
        "Imanol Decima",
        "MarcoCasula",
        "MaríaBohórquez",
        "Renan",
        "IvanL'olivier",
        "Shonero",
        "Luichidev",
        "Faviola Narvaez",
        "Christopher Fuentes",
        "Kuro",
        "Juan Pablo Addeo",
        "Sergio Martínez",
        "Fran Enriquez González",
        "Diana",
        "tictools",
        "ConchaAsensio",
        "Emilio_Arreaza",
        "novamix",
        "Tomas Duclos",
        "Elaya",
        "Ignacio Palominos",
        "David C.",
        "Gerardo Felipe Conrado",
        "ElXuri",
        "David Borja Martinez",
        "JaviFelices",
        "CarlesSànchez",
        "Gorusuke",
        "DavidFabian",
        "ItziarZG",
        "edy WOLF",
        "MarcosGaPe",
        "Jose Jimenez",
        "Borja ",
        "Jhonathan Izquierdo Higuita",
        "MiLfeR322",
        "Sebastián Espínola",
        "Matias Luna",
        "Imanol Decima",
        "MarcoCasula",
        "MaríaBohórquez",
        "Renan",
        "IvanL'olivier",
        "Shonero",
        "Luichidev",
        "Faviola Narvaez",
        "Christopher Fuentes",
        "Kuro",
        "Juan Pablo Addeo",
        "Sergio Martínez",
        "Fran Enriquez González",
        "Diana",
        "tictools",
        "ConchaAsensio",
        "Emilio_Arreaza",
        "novamix",
        "Tomas Duclos",
        "Elaya",
        "Ignacio Palominos",
        "David C.",
        "Gerardo Felipe Conrado",
        "ElXuri",
        "David Borja Martinez",
        "JaviFelices",
        "CarlesSànchez",
        "Gorusuke",
        "DavidFabian",
        "ItziarZG",
        "edy WOLF",
        "MarcosGaPe",
        "Jose Jimenez",
        "Borja ",
        "Jhonathan Izquierdo Higuita",
        "MiLfeR322",
        "Sebastián Espínola",
        "Matias Luna",
        "Imanol Decima",
        "MarcoCasula",
        "MaríaBohórquez",
        "Renan",
        "IvanL'olivier",
        "Shonero",
        "Luichidev",
        "Faviola Narvaez",
        "Christopher Fuentes",
        "Kuro",
        "Juan Pablo Addeo",
        "Sergio Martínez",
        "Fran Enriquez González",
        "Diana",
        "tictools",
        "ConchaAsensio",
        "Emilio_Arreaza",
        "novamix",
        "Tomas Duclos",
        "Elaya",
        "Ignacio Palominos",
        "David C.",
        "Gerardo Felipe Conrado",
        "ElXuri",
        "David Borja Martinez",
        "JaviFelices",
        "CarlesSànchez",
    ]);
    let result = solve(input);
    println!("{result}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_test() {
        let input = Vec::from(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
        let result = super::solve(input);
        assert_eq!(result, String::from("4-4"))
    }
}
