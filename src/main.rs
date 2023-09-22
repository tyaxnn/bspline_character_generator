use nannou::prelude::*;
use rand::Rng;
use bspline;
use std::fs::File;
use std::io::Write;
use std::fs::read_to_string;

//seq 
const SEQ : u32 = 14;
const IND1 : u32 = 7;
const IND2 : u32 = 6;

//display settings
const RESOLUTION : u32 = 20;
const SCALING : u32 = 200;
//line alpha value
const ALPHA : u8 = 255;
//Charactor Component (from primitive to complicated)
    //one stroke
    type D4Line = Vec<Vec4>;
    //one charactor
    #[derive(Clone)]
    struct BsCharactor{
        seq : D4Line,
        ind : Vec<D4Line>,
    }
    //one sentence
    type Sentence = Vec<PlaceBsc>;

//define useful struct when generate new charactor
    struct Pinfo{
        seq_num : u32,
        ind_num_v : Vec<u32>,
    }

    struct PlaceBsc{
        place : Vec2,
        bsc : BsCharactor,
    }


//define useful type when output
    //one 2_dementional_line
    type D2Line = Vec<Vec2>;
    //an assembly of many lines
    type Shirataki = Vec<D2Line>;
    //an assembly of many 4d lines
    type D4shirataki = Vec<D4Line>;

//nannou Model
struct Model{
    bsc : BsCharactor,
    bsc_name : String,
}

fn main(){
    nannou::app(model)
        .run();
}

fn model(app: &App) -> Model{
    app.new_window().size(1000,1000).view(view).event(event).build().unwrap();

    let pinfo = Pinfo { seq_num: 7, ind_num_v: vec![4,4] };
    let bsc = create_simple_bsf(pinfo, 0.01); 

    let bsc_name = "".to_string();

    Model { 
        bsc,
        bsc_name,
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {

    match event {
        KeyPressed(Key::Tab) => {
            //change charactor
            let pinfo = Pinfo { seq_num: SEQ, ind_num_v: vec![IND1,IND2] };
            model.bsc = create_simple_bsf(pinfo, 0.01);

            model.bsc_name = "".to_string();
        }
        KeyPressed(Key::Return) => {

            if model.bsc_name == ""{
                println!("set name before save")
            }
            else{
                //save current charactor
                let save_name = format!("./assets/bcf/{}.bcf",&model.bsc_name.trim());
                
                save_bcf(model.bsc.clone(), &save_name);
            }

        }
        KeyPressed(Key::A) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"a");
        }
        KeyPressed(Key::B) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"b");
        }
        KeyPressed(Key::C) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"c");
        }
        KeyPressed(Key::D) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"d");
        }
        KeyPressed(Key::E) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"e");
        }
        KeyPressed(Key::F) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"f");
        }
        KeyPressed(Key::G) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"g");
        }
        KeyPressed(Key::H) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"h");
        }
        KeyPressed(Key::I) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"i");
        }
        KeyPressed(Key::J) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"j");
        }
        KeyPressed(Key::K) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"k");
        }
        KeyPressed(Key::L) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"l");
        }
        KeyPressed(Key::M) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"m");
        }
        KeyPressed(Key::N) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"n");
        }
        KeyPressed(Key::O) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"o");
        }
        KeyPressed(Key::P) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"p");
        }
        KeyPressed(Key::Q) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"q");
        }
        KeyPressed(Key::R) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"r");
        }
        KeyPressed(Key::S) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"s");
        }
        KeyPressed(Key::T) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"t");
        }
        KeyPressed(Key::U) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"u");
        }
        KeyPressed(Key::V) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"v");
        }
        KeyPressed(Key::W) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"w");
        }
        KeyPressed(Key::X) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"x");
        }
        KeyPressed(Key::Y) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"y");
        }
        KeyPressed(Key::Z) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"z");
        }
        KeyPressed(Key::Underline) => {
            model.bsc_name = format!("{}{}",model.bsc_name.clone(),"_");
        }
        KeyPressed(Key::Back) => {
            model.bsc_name.pop();
        }
        _ => {}
    }
}

//display_current_bsc
fn view(app: &App, model: &Model, frame: Frame) {
    
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to dimgray
    frame.clear(WHITE);

    let left = PlaceBsc{place : vec2(-1.,0.),bsc : read_bcf("./assets/bcf/hatiring.bcf")};
    let center = PlaceBsc{place : vec2(0.,0.),bsc : model.bsc.clone()};
    let right = PlaceBsc{place : vec2(1.,0.),bsc : read_bcf("./assets/bcf/rlike.bcf")};

    let sentence : Sentence = vec![left,center,right];

    //let shirataki = convert_bsc_2_shirataki(model.bsc.clone());
    let shirataki = convert_sentence_2_shirataki(sentence);

    //drawing area 
    draw.rect()
        .w_h(SCALING as f32, SCALING as f32)
        .color(LIGHTGRAY);


    //draw shirataki
    for j in shirataki.clone() {
        let iterator_points = for_pointes_colored(&j,SCALING as f32).into_iter();

        draw.path()
            .stroke()
            .end_cap_round()
            .weight(1.)
            .points_colored(iterator_points);

    }


    //draw text
    draw.text(
            format!("{}",model.bsc_name.trim())
                .as_str()
    )
        .x_y(0.,SCALING as f32 * 0.5 + 10.)
        .color(DIMGRAY);

    draw.text("Tab : Update, Enter : Save")
        .x_y(0.,SCALING as f32 * -0.5 - 10.)
        .color(DIMGRAY);


    


    draw.to_frame(app, &frame).unwrap();
}


//completely randomly create bsc
fn create_simple_bsf(pinfo : Pinfo,v_strength : f32) -> BsCharactor{

    let mut seq : D4Line = Vec::new();
    let mut ind : Vec<D4Line> = Vec::new();
    //create seq
    for _ in 0..pinfo.seq_num{
        seq.push(random_d4(v_strength));
    }

    if seq[seq.len()-1].x < seq[0].x{
        seq.reverse();
    }
    //create ind
    for ind_num in pinfo.ind_num_v{
        let mut a_ind : D4Line = Vec::new();
        for _ in 0..ind_num{
            a_ind.push(random_d4(v_strength));
        }

        ind.push(a_ind);
    }

    BsCharactor {
        seq,
        ind,
    }
}

//a function create random 2d vector
fn random_vec2(max_vec2_norm : f32) -> Vec2{
    //let normal = Normal::new(0.0, max_vec2_norm).unwrap();

    let mut rng = rand::thread_rng();

    let x = (rng.gen::<f32>() - 0.5) * 2. * max_vec2_norm;
    let y = (rng.gen::<f32>() - 0.5) * 2. * max_vec2_norm;
    let rand_vec = vec2(x,y);

    rand_vec
}

//create random vec4
    //coordinate center (0.5,0.5),range[0,1]
    //verocity range[-vmax,vmax]
fn random_d4(v_strength : f32)-> Vec4{
    let coordinate = random_vec2(0.5) + vec2(0.5,0.5);

    let verosity = random_vec2(v_strength);

    vec4(coordinate.x,coordinate.y,verosity.x,verosity.y)
}

//convert d2line so that nannou can draw line
fn for_pointes_colored(input : &D2Line, scaling : f32) ->  Vec<(Vec2,Rgba8)> {
    let mut out_put = Vec::new();
    for i in input {

        let x_nannou = (i.x - 0.5) * scaling;
        let y_nannou = (i.y - 0.5) * scaling;
        out_put.push((vec2(x_nannou,y_nannou),rgba8(240,160,180,ALPHA)));
    }
    out_put
}

//convert bsc to shirataki ,which is useful for nannou
fn _convert_bsc_2_shirataki(bsc : BsCharactor) -> Shirataki{
    let mut d4shirataki : D4shirataki = Vec::new();

    //bsc pack into 4 dementional shirataki
    d4shirataki.push(bsc.seq.clone());
    for a_ind in bsc.ind.clone(){
        d4shirataki.push(a_ind);
    }

    //make smooth by bspline
    for i in 0..d4shirataki.len(){
        d4shirataki[i] = bspline_for_d4line(&d4shirataki[i]);
    }

    //convert to shirataki
    let shirataki = d4_2_d2(d4shirataki);
    shirataki
}

//convert sentence to shirataki ,which is useful for nannou
fn convert_sentence_2_shirataki(sentence : Sentence) -> Shirataki{
    let mut seq_part : D4Line = Vec::new();
    let mut d4shirataki : D4shirataki = Vec::new();

    //loop for all bsc
    for one_p_bsc in sentence{
        //change coordinate by place
        let place : Vec2 = one_p_bsc.place;
        let seq : D4Line = one_p_bsc.bsc.seq;
        let ind : Vec<D4Line> = one_p_bsc.bsc.ind;

        //update seq
        let mut new_seq = seq.into_iter()
            .map(|v4| v4 + vec4(place.x,place.y,0.,0.,))
            .collect();

        seq_part.append(&mut new_seq);
        //update ind
        for a_ind in ind{
            let new_aind = a_ind.into_iter()
                .map(|v4| v4 + vec4(place.x,place.y,0.,0.,))
                .collect();
            d4shirataki.push(new_aind);
        }
    }

    d4shirataki.push(seq_part);

    //make smooth by bspline
    for i in 0..d4shirataki.len(){
        d4shirataki[i] = bspline_for_d4line(&d4shirataki[i]);
    }

    //convert to shirataki
    let shirataki = d4_2_d2(d4shirataki);
    shirataki
    
}

//convert row (more light) d4line to high (more heavy) d4line by using bspline
fn bspline_for_d4line(row_d4line : &D4Line) -> D4Line {

    //set "knots"
    let mut knots : Vec<f32> = Vec::new();

    knots.push(-2.0 as f32);
    knots.push(-2.0 as f32);

    let points_num = row_d4line.len();

    for i in 0..points_num{
        let para = (i+1) as f32 / points_num as f32 *4.0 -2.0;
        knots.push(para);
    }

    knots.push(2.0 as f32);
    knots.push(2.0 as f32);

    let degree = 3;
    let bspline = bspline::BSpline::new(degree, row_d4line.clone(), knots);

    let mut new_d4line = Vec::new();

    //at here, you excute smoothing
    for i in 0..10*points_num{
        let first = bspline.knot_domain().0;
        let last = bspline.knot_domain().1;
        let para = (last - first) * (i as f32 /(10*points_num) as f32) + first;

        new_d4line.push(bspline.point(para));
    }

    new_d4line
}

//Vec<D4line> to Vec<D2line> by add verosity to coordinate
fn d4_2_d2(d4shirataki : Vec<D4Line>) -> Shirataki {
    let mut shirataki : Shirataki = Vec::new();

    for d4line in d4shirataki{
        for j in 0..(RESOLUTION+1){
            let mut d2line : D2Line = Vec::new();

            let mul = {
                let jf = j as f32;
                let maxf = (RESOLUTION+1) as f32;
                jf / maxf * 10.
            };
            for i in 0..d4line.len(){
                d2line.push(
                    vec2(
                        d4line[i].x + d4line[i].z * mul,
                        d4line[i].y + d4line[i].w * mul,
                    )
                );
            }

            shirataki.push(d2line);
        }
    }

    shirataki
}

//convert bsc into string(for save as .bcf)
fn bsc_2_bcf(bsc : BsCharactor) -> String{
    let mut str_out = "".to_string();

    str_out = format!("{}{}",str_out,"seq");
    for v4 in bsc.seq{
        str_out = format!("{}\n{} {} {} {}",str_out,v4.x,v4.y,v4.z,v4.w);
    }
    for i in 0..bsc.ind.len(){
        str_out = format!("{}\n{}",str_out,"ind");
        for v4 in bsc.ind[i].clone(){
            str_out = format!("{}\n{} {} {} {}",str_out,v4.x,v4.y,v4.z,v4.w);
        }
    }
    
    str_out
}

fn save_bcf(bsc : BsCharactor, path : &str){
    let bcf = bsc_2_bcf(bsc);

    let mut file = File::create(path).unwrap();
                let _ = file.write_all(
                    bcf.as_bytes()
                );

    println!("saved successfully")
}

//convert String into bsc(for open ,bcf)
fn bcf_2_bsc(bcf : String) -> BsCharactor{
    let lines = bcf.lines();

    let mut seq : D4Line = Vec::new();
    let mut a_ind : D4Line = Vec::new();
    let mut ind : Vec<D4Line> = Vec::new();

    let mut reach_ind : bool = false;

    

    for each_line in lines{
        if each_line == "seq"{}
        else if each_line == "ind"{
            if reach_ind{
                ind.push(a_ind.clone());
                a_ind.clear();
            }
            else{
                reach_ind = true;
            }
        }
        else if reach_ind == false{
            let mut v_f_v4 : Vec<f32> = Vec::new();

            let num_strings = each_line.split_whitespace();


            for num_string in num_strings{
                let num : f32 = num_string.parse().expect("not a number");

                v_f_v4.push(num);
            }

            seq.push(vec4(v_f_v4[0],v_f_v4[1],v_f_v4[2],v_f_v4[3]));

        }
        else{
            let mut v_f_v4 : Vec<f32> = Vec::new();

            let num_strings = each_line.split_whitespace();


            for num_string in num_strings{
                let num : f32 = num_string.parse().expect("not a number");

                v_f_v4.push(num);
            }

            a_ind.push(vec4(v_f_v4[0],v_f_v4[1],v_f_v4[2],v_f_v4[3]));
        }
    }

    BsCharactor { seq, ind }

}

fn read_bcf(path : &str) -> BsCharactor{
    let bcf : String = read_to_string(path).expect("cannnot open the file");

    bcf_2_bsc(bcf)
}