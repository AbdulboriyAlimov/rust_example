fn main() {
    let point_set:Vec<f64> = Vec::from([4.,2.,1.,6.,3.,8.,10.,9.]);
    
    let sorted = merge_sertion(&point_set);
    
    println!("{:?}",sorted);
}

fn merge_sertion(points:&Vec<f64>) ->Vec<f64> {
    let size = points.len();
    
    if points.len() <=7 {
        return insertion(points);
    } else {
        let left = merge_sertion(&points[0..size/2].to_vec());
        let right = merge_sertion(&points[size/2..size].to_vec());
        
        let mut final_set:Vec<f64> = Vec::with_capacity(size);
        let mut i =0;
        let mut j =0;
        
        for _k in 0..size {
            if left[i]<=right[j] {
                final_set.push(left[i]);
                i+=1;
                if i==left.len() {
                    final_set.extend(right[j..right.len()].to_vec());
                    break
                }
            } else {
                final_set.push(right[j]);
                j+=1;
                if j==right.len() {
                    final_set.extend(left[i..left.len()].to_vec());
                }
            }
        }
        
        return final_set;
    }
}

fn insertion(points:&Vec<f64>) ->Vec<f64>{
    let mut sub_set = points.clone();

    for j in 1..points.len() {
        let key = points[j];
        let mut i:usize = j-1;
        
        while (i>0||i==0) && points[i] >key {
            sub_set[i + 1usize] = points[i];
            i = i-1;
        }
        sub_set[i + 1usize] = key;
    }
    return sub_set.to_vec();
}
