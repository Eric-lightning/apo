use std::string::String;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub fn to_json(apo_datas: &mut BTreeMap<i64,HashMap<&str,String>>) -> String {
        let mut json_str = String::new();
        json_str += "{\n";
        // TODO:キーなしHashMapに変更。
        let jb_open  = "\": {\n";
        let jb_close = "\n},";
        let jb_line = ",\n";
        let dquote  = "\"";
        for (_key,val) in apo_datas {
            json_str += dquote;
            json_str += val.get("time").unwrap();
            json_str += jb_open;
            json_str += "\"type\": ";
            json_str += dquote;
            json_str += val.get("type").unwrap();
            json_str += dquote;
            json_str += jb_line;
            json_str += "\"important\": ";
            json_str += val.get("important").unwrap();
            json_str += jb_line;
            json_str += "\"recurse\": ";
            json_str += val.get("recurse").unwrap();
            json_str += jb_line;
            json_str += "\"texts\": ";
            json_str += dquote;
            json_str += val.get("texts").unwrap();
            json_str += dquote;
            json_str += jb_close;
        }
        json_str.pop();
        json_str += "\n}";
        return json_str;
}
pub fn to_csv(apo_datas: &mut BTreeMap<i64,HashMap<&str,String>>) -> String {
        let coma = ",";
        let line = "\n";
        let dqte = "\"";
        let mut csv_str = "TIME,TYPE,IMPORTANT,RECURSE,TEXTS\n".to_string();
        for (_key,val) in apo_datas {
            csv_str += val.get("time").unwrap();
            csv_str += coma;
            csv_str += val.get("type").unwrap();
            csv_str += coma;
            csv_str += val.get("important").unwrap();
            csv_str += coma;
            csv_str += val.get("recurse").unwrap();
            csv_str += coma;
            csv_str += dqte;
            csv_str += val.get("texts").unwrap();
            csv_str += dqte;
            csv_str += line;
        }
        return csv_str;
}

pub fn to_tsv(apo_datas: &mut BTreeMap<i64,HashMap<&str,String>>) -> String {
        let tab = "\t";
        let line = "\n";
        let dqte = "\"";
        let mut tsv_str = "TIME\tTYPE\tIMPORTANT\tRECURSE\tTEXTS\n".to_string();
        for (_key,val) in apo_datas {
            tsv_str += val.get("time").unwrap();
            tsv_str += tab;
            tsv_str += val.get("type").unwrap();
            tsv_str += tab;
            tsv_str += val.get("important").unwrap();
            tsv_str += tab;
            tsv_str += val.get("recurse").unwrap();
            tsv_str += tab;
            tsv_str += dqte;
            tsv_str += val.get("texts").unwrap();
            tsv_str += dqte;
            tsv_str += line;
        }
        return tsv_str;
}
