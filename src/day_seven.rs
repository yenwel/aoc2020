use super::utils;
use nom;
use petgraph;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
struct BagColour {    
	adjective: String,
	colour: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BagRule {
	bag: BagColour,
	contained_bags: Vec<(usize,BagColour)>,
}

fn bag_colour(bag_colour: &str) -> nom::IResult<&str, BagColour>{
    match nom::sequence::separated_pair(
        nom::character::complete::alpha1, 
        nom::bytes::complete::tag(" "), 
        nom::character::complete::alpha1)(bag_colour) {
        Ok((remainder,(adjective, colour))) => Ok((remainder, BagColour {
        adjective: adjective.to_string(),
        colour : colour.to_string()})),
        Err(e) => Err(e)
    }
}

fn bag(bag: &str) -> nom::IResult<&str, BagColour>{
    match nom::sequence::tuple((
        bag_colour,
        nom::character::complete::space1,
        nom::bytes::complete::tag("bag"),
        nom::combinator::opt(nom::bytes::complete::tag("s"))
    ))(bag){
        Ok((remainder,(bag_colour, _,_,_))) => Ok((remainder,bag_colour)),
        Err(e) => Err(e)
    }
}

fn contained_bags(contained_bags: &str) ->  nom::IResult<&str, Vec<(usize,BagColour)>> {
    if contained_bags == "no other bags."
    {
        return Ok((".",[].to_vec()))
    }
    match 
    nom::multi::separated_nonempty_list(
        nom::bytes::complete::tag(", "),
        nom::sequence::tuple((
            nom::character::complete::digit1,
            nom::character::complete::space1,
            bag,            
        ))
    )(contained_bags)
    {
        Ok((remainder,matches)) => Ok((remainder,matches.iter().map(|(number,_, bag_colour)|(number.parse::<usize>().unwrap(),bag_colour.clone())).collect())),
        Err(e) => Err(e)
    }
}

fn bag_rule_parser(line: &str) -> nom::IResult<&str,BagRule>{
    match nom::sequence::tuple(
        (bag,
        nom::bytes::complete::tag(" contain "),
        contained_bags,
        nom::bytes::complete::tag(".")
        ))(line)
    {
        Ok((remainder,(bag, _ ,contained_bags,_))) =>
            Ok((remainder,BagRule{
                bag:bag,
                contained_bags :contained_bags
            })),
        Err(e) => Err(e)
    }
}

fn format_bag(bag: &BagColour )-> &'static str {
    let s: String = format!("{}-{}",bag.adjective,bag.colour);
    Box::leak(s.into_boxed_str())
}

fn build_bag_graph(input: &str)-> petgraph::graphmap::DiGraphMap<&str,usize> {
    let edges: Vec<(&str, &str,usize)> = input
        .lines()
        .map(|line| bag_rule_parser(line))
        .map(|result| result.unwrap())
        .map(|(_, rule)| (rule.bag, rule.contained_bags))
        .flat_map(
            |(root_bag, contained_bags)| 
            contained_bags
            .iter()
            .map(|(size, bag)|
                (
                    root_bag.clone(),
                    bag.clone(),
                    (*size).to_owned()
                )
            )
            .collect::<Vec<(BagColour,BagColour, usize)>>()
        )
        .map(move|(root_bag,bag,size)|(format_bag(&root_bag), format_bag(&bag),size)).collect();
    petgraph::graphmap::DiGraphMap::<&str, usize>::from_edges(edges)    
}

fn build_bag_reverse_graph(input: &str)-> petgraph::graphmap::DiGraphMap<&str,usize> {
    let edges: Vec<(&str, &str,usize)> = input
        .lines()
        .map(|line| bag_rule_parser(line))
        .map(|result| result.unwrap())
        .map(|(_, rule)| (rule.bag, rule.contained_bags))
        .flat_map(
            |(root_bag, contained_bags)| 
            contained_bags
            .iter()
            .map(|(size, bag)|
                (
                    root_bag.clone(),
                    bag.clone(),
                    (*size).to_owned()
                )
            )
            .collect::<Vec<(BagColour,BagColour, usize)>>()
        )
        .map(move|(root_bag,bag,size)|(format_bag(&root_bag), format_bag(&bag),size)).collect();
    petgraph::graphmap::DiGraphMap::<&str, usize>::from_edges(edges)    
}

fn search_bag_graph_for_gold<'a>(g: &'a petgraph::graphmap::DiGraphMap<&str,usize>) -> Vec<&'a str> {
    let mut result : Vec<&'a str> = [].to_vec();
    petgraph::visit::depth_first_search(&g,Some("shiny-gold"),|event|{
        if let petgraph::visit::DfsEvent::TreeEdge(u, v) = event {
            println!("{} {}",u,v );
            result.push(v)
        }
        petgraph::visit::Control::<&str>::Continue
    });
    result
}

fn count_contained_bags_recur(g: &petgraph::graphmap::DiGraphMap<&str,usize>, node: &str) -> usize {
    g.neighbors_directed(node, petgraph::prelude::Outgoing)
    .map(|child_node|                
        if let Some(weight) = g.edge_weight(node,child_node) {
            let count_childs = count_contained_bags_recur(g, child_node);            
            println!("{}-{}->{} count_childs {}",node, weight,child_node, count_childs);
            weight + (weight * count_childs)
        } else
        {
            1
        }
    ).sum()
}

pub fn run_challenge_one()
{
    let input = utils::read_day(7);
    match input {
        Ok(input) => println!("{:?}", search_bag_graph_for_gold(&build_bag_graph(&input)).iter().count()),
        Err(_) => println!("error")
    }    
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(7);
   match input {
        Ok(input) => println!("{}",count_contained_bags_recur(&build_bag_reverse_graph(&input), "shiny-gold")),
        Err(_) => println!("error")
    }   

}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::super::utils;

    #[test]
    fn test_parsers() {
        assert_eq!(bag_colour("shiny gold"), Ok(("", BagColour {
                    adjective: "shiny".to_string(),
                    colour : "gold".to_string()})));               
        assert_eq!(bag("shiny gold bag"), Ok(("", BagColour {
                        adjective: "shiny".to_string(),
                        colour : "gold".to_string()})));                    
        assert_eq!(bag("shiny gold bags"), Ok(("", BagColour {
            adjective: "shiny".to_string(),
            colour : "gold".to_string()})));
        assert_eq!(contained_bags("1 bright white bag, 2 muted yellow bags"),
            Ok(("", 
                [
                    (1, BagColour { adjective: "bright".to_string(), colour : "white".to_string()}), 
                    (2, BagColour { adjective: "muted".to_string(), colour: "yellow".to_string() })].to_vec())));
        
        assert_eq!(bag_rule_parser("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        Ok(("", BagRule {
            bag : BagColour { adjective: "light".to_string(), colour : "red".to_string()},
            contained_bags : [
                (1, BagColour { adjective: "bright".to_string(), colour : "white".to_string()}), 
                (2, BagColour { adjective: "muted".to_string(), colour: "yellow".to_string() })].to_vec()})))
    }

    #[test]
    fn test_challenge_one() {
        let input = utils::read_test(7, None);
        match input {
            Ok(input) =>{  
                println!("{:?}",build_bag_graph(&input));
                assert_eq!(search_bag_graph_for_gold(&build_bag_graph(&input)),["dark-olive", "faded-blue", "dotted-black", "vibrant-plum"])
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two() {
        let input = utils::read_test(7, None);
        match input {
            Ok(input) =>{  
                
                println!("{:?}",build_bag_reverse_graph(&input));
                assert_eq!(count_contained_bags_recur(&build_bag_reverse_graph(&input),"shiny-gold"),32)
            }
            Err(_) => assert!(false)
        }        
    }

    
    #[test]
    fn test_challenge_two_nested() {
        let input = utils::read_test(7, Some("nested"));
        match input {
            Ok(input) =>{  
                
                println!("{:?}",build_bag_reverse_graph(&input));
                assert_eq!(count_contained_bags_recur(&build_bag_reverse_graph(&input),"shiny-gold"),126)
            }
            Err(_) => assert!(false)
        }        
    }
}