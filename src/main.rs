use nannou::prelude::*;
use std::cmp::min;

mod bsc;
use bsc::*;
use bsc::generate_bsc::create_simple_bsf;
use bsc::generate_shirataki::convert_sentence_2_shirataki;
use bsc::read_bcf::read_bcf;
use bsc::write_bcf::write_bcf;

//generate setting 
const SEQ : u32 = 14;
const IND1 : u32 = 6;
const IND2 : u32 = 4;

//display settings
const RESOLUTION : u32 = 30;
const SCALING : u32 = 350;
const COMPRESS_X : f32 = 0.5;
const BOLDNESS : f32 = 5.;
//line alpha value
const ALPHA : u8 = 255;


//nannou Model
struct Model{
    bsc : BsCharactor,
    bsc_name : String,
    mode : Mode,
}

enum Mode{
    View,
    Seq(Option<i32>),
    Ind(Option<i32>,Option<i32>),
}

fn main(){
    nannou::app(model)
        .run();
}

fn model(app: &App) -> Model{
    app.new_window().size((4.*COMPRESS_X)as u32 * SCALING,4 * SCALING).view(view).event(event).build().unwrap();

    let pinfo = Pinfo { seq_num: SEQ, ind_num_v: vec![IND1,IND2] };
    let bsc = create_simple_bsf(pinfo, 0.01); 

    let bsc_name = "".to_string();

    let mode = Mode::View;

    Model { 
        bsc,
        bsc_name,
        mode,
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {

    match model.mode {
        Mode::View => {
            match event{
                KeyPressed(Key::Return) => {
                    let open_name = format!("./assets/bcf/{}.bcf",&model.bsc_name.trim());
                    match read_bcf(&open_name){
                        None => {println!("file not exists")}
                        _ => {
                            model.bsc = read_bcf(&open_name).unwrap()
                        }
                    }
                }
                KeyPressed(Key::F1) => {
                    //change charactor
                    let pinfo = Pinfo { seq_num: SEQ, ind_num_v: vec![IND1,IND2] };
                    model.bsc = create_simple_bsf(pinfo, 0.01);

                    model.bsc_name = "".to_string();
                }
                KeyPressed(Key::F12) => {

                    if model.bsc_name == ""{
                        println!("set name before save")
                    }
                    else{
                        //save current charactor
                        let save_name = format!("./assets/bcf/{}.bcf",&model.bsc_name.trim());
                        
                        write_bcf(model.bsc.clone(), &save_name);
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
                KeyPressed(Key::Key1) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"1");
                }
                KeyPressed(Key::Key2) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"2");
                }
                KeyPressed(Key::Key3) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"3");
                }
                KeyPressed(Key::Key4) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"4");
                }
                KeyPressed(Key::Key5) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"5");
                }
                KeyPressed(Key::Key6) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"6");
                }
                KeyPressed(Key::Key7) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"7");
                }
                KeyPressed(Key::Key8) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"8");
                }
                KeyPressed(Key::Key9) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"9");
                }
                KeyPressed(Key::Key0) => {
                    model.bsc_name = format!("{}{}",model.bsc_name.clone(),"0");
                }
                KeyPressed(Key::Back) => {
                    model.bsc_name.pop();
                }
                KeyPressed(Key::Space) => {
                    model.mode = Mode::Seq(None)
                }            
                _ => {}
            }
        }
        Mode::Seq(None) => {
            match event{
                KeyPressed(Key::Space) => {
                    model.mode = Mode::Ind(None,None)
                }
                KeyPressed(Key::Tab) => {
                    let seq_len = model.bsc.seq.len() as i32;
                    if 0 < seq_len {
                        model.mode = Mode::Seq(Some(0))
                    }
                    else {}
                }
                _ => {}
            }
        }
        Mode::Seq(other) => {
            let num = other.unwrap();
            match event{
                KeyPressed(Key::Tab) => {
                    let seq_len = model.bsc.seq.len() as i32;
                    model.mode = Mode::Seq(Some((num + 1) % seq_len))
                }
                KeyPressed(Key::Space) => {
                    model.mode = Mode::Ind(None,None)
                }
                KeyPressed(Key::Up) => {
                    model.bsc.seq[num as usize].y += 0.01
                }
                KeyPressed(Key::Down) => {
                    model.bsc.seq[num as usize].y -= 0.01
                }
                KeyPressed(Key::Left) => {
                    model.bsc.seq[num as usize].x -= 0.01
                }
                KeyPressed(Key::Right) => {
                    model.bsc.seq[num as usize].x += 0.01
                }
                KeyPressed(Key::W) => {
                    model.bsc.seq[num as usize].w += 0.0001
                }
                KeyPressed(Key::S) => {
                    model.bsc.seq[num as usize].w -= 0.0001
                }
                KeyPressed(Key::A) => {
                    model.bsc.seq[num as usize].z -= 0.0001
                }
                KeyPressed(Key::D) => {
                    model.bsc.seq[num as usize].z += 0.0001
                }

                _ => {}
            }
        }
        Mode::Ind(None,_) => {
            match event{
                KeyPressed(Key::Space) => {
                    model.mode = Mode::View
                }
                KeyPressed(Key::Tab) => {
                    let ind_len = model.bsc.ind.len() as i32;
                    if 0 < ind_len {
                        model.mode = Mode::Ind(Some(0),None)
                    }
                    else {}
                }
                _ => {}
            }
        }
        Mode::Ind(ind_op,None) => {
            let ind_num = ind_op.unwrap();
            match event{
                KeyPressed(Key::Space) => {
                    model.mode = Mode::View
                }
                KeyPressed(Key::Tab) => {
                    let ind_len = model.bsc.ind.len() as i32;
                    model.mode = Mode::Ind(Some((ind_num + 1) % ind_len),None)
                }
                KeyPressed(Key::Return) => {
                    let a_ind_len = model.bsc.ind[ind_op.unwrap() as usize].len() as i32;
                    if 0 < a_ind_len {
                        model.mode = Mode::Ind(ind_op,Some(0))
                    }
                    else {}
                }
                _ => {}
            }
        }
        Mode::Ind(ind_op,num_op) => {
            let ind_num = ind_op.unwrap();
            let num = num_op.unwrap();
            match event{
                KeyPressed(Key::Tab) => {
                    let a_ind_len = model.bsc.ind[ind_num as usize].len() as i32;
                    model.mode = Mode::Ind(ind_op,Some((num + 1) % a_ind_len))
                }
                KeyPressed(Key::Space) => {
                    model.mode = Mode::View
                }
                KeyPressed(Key::Up) => {
                    model.bsc.ind[ind_num as usize][num as usize].y += 0.01
                }
                KeyPressed(Key::Down) => {
                    model.bsc.ind[ind_num as usize][num as usize].y -= 0.01
                }
                KeyPressed(Key::Left) => {
                    model.bsc.ind[ind_num as usize][num as usize].x -= 0.01
                }
                KeyPressed(Key::Right) => {
                    model.bsc.ind[ind_num as usize][num as usize].x += 0.01
                }
                KeyPressed(Key::W) => {
                    model.bsc.ind[ind_num as usize][num as usize].w += 0.0001
                }
                KeyPressed(Key::S) => {
                    model.bsc.ind[ind_num as usize][num as usize].w -= 0.0001
                }
                KeyPressed(Key::A) => {
                    model.bsc.ind[ind_num as usize][num as usize].z -= 0.0001
                }
                KeyPressed(Key::D) => {
                    model.bsc.ind[ind_num as usize][num as usize].z += 0.0001
                }
                KeyPressed(Key::Back) => {
                    model.mode = Mode::Ind(ind_op,None)
                }
                _ => {}
            }
        }
    }
}

//display_current_bsc
fn view(app: &App, model: &Model, frame: Frame) {
    
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to dimgray
    frame.clear(WHITE);

    let solo = PlaceBsc{place : vec2(0.,-0.),bsc : model.bsc.clone()};
    let left = PlaceBsc{place : vec2(-1. * COMPRESS_X,-1.2),bsc : read_bcf("./assets/bcf/f00010001.bcf").unwrap()};
    let center = PlaceBsc{place : vec2(0.,-1.2),bsc : model.bsc.clone()};
    let right = PlaceBsc{place : vec2(1. * COMPRESS_X,-1.2),bsc : read_bcf("./assets/bcf/f00010002.bcf").unwrap()};

    let sentence : Sentence = vec![solo,left,center,right];

    //let shirataki = convert_bsc_2_shirataki(model.bsc.clone());
    let shirataki : Shirataki = convert_sentence_2_shirataki(sentence,RESOLUTION,BOLDNESS,COMPRESS_X);

    
    //drawing area 
    draw.rect()
        .w_h(SCALING as f32 * COMPRESS_X, SCALING as f32)
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

    match model.mode{
        Mode::View => {
            draw.text("Space : Change Mode, F1 : Generate, F12 : Save, Enter : Open, AtoZ : Change Name, Back : Delete type")
            .x_y(0.,SCALING as f32 * -0.5 - 10.)
            .w(500.)
            .color(DIMGRAY);
        }
        Mode::Seq(None) => {
            draw.text("Space : Change Mode, Tab : Change Point")
            .x_y(0.,SCALING as f32 * -0.5 - 10.)
            .w(500.)
            .color(DIMGRAY);
        }
        Mode::Seq(_) => {
            draw.text("Space : Change Mode, Tab : Change Point, Arrows : Move Point")
            .x_y(0.,SCALING as f32 * -0.5 - 10.)
            .w(500.)
            .color(DIMGRAY);
        }
        Mode::Ind(None, _) => {
            draw.text("Space : Change Mode, Tab : Change the Ind")
            .x_y(0.,SCALING as f32 * -0.5 - 10.)
            .w(500.)
            .color(DIMGRAY);
        }
        Mode::Ind(_, None) => {
            draw.text("Space : Change Mode, Tab : Change the Ind, Enter : Go to Children")
            .x_y(0.,SCALING as f32 * -0.5 - 10.)
            .w(500.)
            .color(DIMGRAY);
        }
        Mode::Ind(_, _) => {
            draw.text("Space : Change Mode, Tab : Change Point, Arrows : Move Point, Back : Back to Parent")
            .x_y(0.,SCALING as f32 * -0.5 - 10.)
            .w(500.)
            .color(DIMGRAY);
        }
    }
    

    let display_place = vec2(0.,SCALING as f32 * 0.5 + 0.5 + 30.);

    match model.mode{

        Mode::View => {
            draw.text("View Mode")
                .xy(display_place)
                .color(DIMGRAY);
        }
        Mode::Seq(None) => {
            draw.text("Edit Mode Seq")
                .xy(display_place)
                .color(DIMGRAY);
        }
        Mode::Seq(other) => {
            draw.text(format!("Edit Mode Seq[{}]",other.unwrap()).as_str())
                .xy(display_place)
                .color(DIMGRAY);
        }
        Mode::Ind(None, _) => {
            draw.text("Edit Mode Ind")
                .xy(display_place)
                .color(DIMGRAY);
        }
        Mode::Ind(ind_op, None) => {
            draw.text(format!("Edit Mode Ind[{}]",ind_op.unwrap()).as_str())
                .xy(display_place)
                .color(DIMGRAY);
        }
        Mode::Ind(ind_op, num_op) => {
            draw.text(format!("Edit Mode Ind[{}][{}]",ind_op.unwrap(),num_op.unwrap()).as_str())
                .xy(display_place)
                .color(DIMGRAY);
        }
    }

    //draw point
    let seq_points : Vec<Vec4> = model.bsc.seq.clone();

    for i in 0..seq_points.len(){
        let x_nannou = ((seq_points[i].x-0.5) * COMPRESS_X) * SCALING as f32;
        let y_nannou = (seq_points[i].y - 0.5) * SCALING as f32;

        let coor = vec2(x_nannou,y_nannou);

        let vx_nannou = seq_points[i].z * COMPRESS_X * SCALING as f32 * 10.;
        let vy_nannou = seq_points[i].w * SCALING as f32 * 10.;

        let vero = vec2(vx_nannou,vy_nannou);

        draw.ellipse()
            .xy(coor)
            .color(ORANGE)
            .w(7.0)
            .h(7.0);

        draw.arrow()
            .points(coor, coor+vero)
            .weight(2.)
            .color(ORANGE);

        draw.text(format!("{}",i).as_str())
            .x_y(x_nannou,y_nannou+ 14.)
            .font_size(20)
            .color(ORANGE);
    }

    for i in 0..model.bsc.ind.len(){
        let ind_points : Vec<Vec4> = model.bsc.ind[i].clone();
        for j in 0..ind_points.len(){

            let x_nannou = ((ind_points[j].x-  0.5) * COMPRESS_X) * SCALING as f32;
            let y_nannou = (ind_points[j].y - 0.5) * SCALING as f32;

            let coor = vec2(x_nannou,y_nannou);

            let vx_nannou = ind_points[j].z * COMPRESS_X * SCALING as f32 * 10.;
            let vy_nannou = ind_points[j].w * SCALING as f32 * 10.;

            let vero = vec2(vx_nannou,vy_nannou);

            draw.ellipse()
                .xy(coor)
                .color(rgba8(min(255,170 - i * 85) as u8,170,min(255,i * 100) as u8,255))
                .w(7.0)
                .h(7.0);

            draw.arrow()
                .points(coor, coor+vero)
                .weight(2.)
                .color(rgba8(min(255,170 - i * 85) as u8,170,min(255,i * 100) as u8,255));

            draw.text(format!("{}",j).as_str())
                .x_y(x_nannou,y_nannou+ 14.)
                .color(rgba8(min(255,170 - i * 85) as u8,170,min(255,i * 100) as u8,255))
                .font_size(20);
        }

    }

    draw.to_frame(app, &frame).unwrap();
}




//convert d2line so that nannou can draw line
fn for_pointes_colored(input : &D2Line, scaling : f32) ->  Vec<(Vec2,Rgba8)> {
    let mut out_put = Vec::new();
    for i in input {

        let x_nannou = (i.x - 0.5) * scaling;
        let y_nannou = (i.y - 0.5) * scaling;
        out_put.push((vec2(x_nannou,y_nannou),rgba8(170,170,170,ALPHA)));
    }
    out_put
}