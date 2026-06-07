use slint::Model;

slint::slint! {
    import { VerticalBox, HorizontalBox, Button, LineEdit, ScrollView } from "std-widgets.slint";
    import { AboutSlint } from "std-widgets.slint";

    // 全パーツの状態を一元管理する共通構造体
    export struct ComponentState {
        id: int,
        title: string,
        comp-type: string,
        is-expanded: bool,
        is-visible: bool,
    }

    // ─── 【共通フレーム：あなたが100%理解した美しい器】 ───
    component CustomWidgetFrame inherits Rectangle {
        in property <string> title: "機能パーツ";
        in-out property <bool> is-expanded: true;
        in property <bool> is-selected: false;

        callback toggle-size(); callback move-up(); callback move-down();
        callback hide-component(); callback handle-clicked();

        width: root.is-expanded ? 440px : 210px;
        height: root.is-expanded ? 120px : 80px;
        background: root.is-selected ? #eaf2f8 : #ffffff;
        border-radius: 8px; border-width: 2px;
        border-color: root.is-selected ? #3498db : #bdc3c7;

        animate width { duration: 150ms; easing: ease-in-out; }
        animate height { duration: 150ms; easing: ease-in-out; }

        HorizontalBox {
            padding: 4px; spacing: 6px;
            
            // 左側の2クリック式ハンドルエリア
            Rectangle {
                width: 35px; background: root.is-selected ? #3498db : #f0f3f6; border-radius: 4px;
                Text { text: root.is-selected ? "OK" : "::"; font-size: 14px; font-weight: 700; color: root.is-selected ? #ffffff : #7f8c8d; horizontal-alignment: center; vertical-alignment: center; }
                TouchArea { clicked => { root.handle-clicked(); } }
            }
            
            // 右側の共通ボタンとコンテンツエリア
            VerticalBox {
                padding: 4px; spacing: 4px;
                HorizontalBox {
                    padding: 0;
                    Text { text: root.title; font-weight: 700; font-size: 13px; color: #2c3e50; vertical-alignment: center; }
                    HorizontalBox {
                        alignment: end; spacing: 3px; padding: 0;
                        Button { text: "↑"; width: 25px; clicked => { root.move-up() } }
                        Button { text: "↓"; width: 25px; clicked => { root.move-down() } }
                        Button { text: root.is-expanded ? "縮" : "広"; width: 30px; clicked => { root.toggle-size() } }
                        Button { text: "X"; width: 25px; clicked => { root.hide-component() } }
                    }
                }
                @children
            }
        }
    }

    // ─── 各専門コンポーネント群（中身の関数配列） ───
    component ZandakaComp inherits VerticalBox { Text { text: "現在の想定総残高: ¥4,567,890"; font-size: 15px; color: #27ae60; font-weight: 700; } }
    component ClockComp inherits VerticalBox { in property <string> time; Text { text: root.time; font-size: 18px; color: #2980b9; font-weight: 700; } }
    component BoxComp inherits VerticalBox { in property <string> date; Text { text: root.date; font-size: 14px; color: #7f8c8d; } }
    component MemoComp inherits VerticalBox { LineEdit { placeholder-text: "重要メモをここに書き留める..."; } }
    component AdComp inherits VerticalBox { Rectangle { background: #fdf2e9; Text { text: "[広告枠] 最新のRust学習書、好評発売中！"; color: #e67e22; font-size: 11px; } } }
    component TenkiComp inherits VerticalBox { HorizontalBox { alignment: start; spacing: 10px; Text { text: "Fine"; font-size: 18px; font-weight: 700; color: #e67e22; } VerticalBox { Text { text: "東京の天気: 晴れ"; font-size: 14px; font-weight: 700; } Text { text: "気温: 26度 / 降水確率: 10%"; font-size: 11px; color: #7f8c8d; } } } }
    component KabukaComp inherits VerticalBox { Text { text: "日経平均株価: ¥38,500 (+250)"; font-size: 14px; color: #e74c3c; font-weight: 700; } HorizontalBox { spacing: 5px; alignment: start; Rectangle { background: #e74c3c; width: 20px; height: 10px; } Rectangle { background: #e74c3c; width: 20px; height: 20px; } Rectangle { background: #e74c3c; width: 20px; height: 35px; } } }
    component TodoComp inherits VerticalBox { spacing: 3px; Text { text: "[ ] 1. 経費精算を終わらせる"; font-size: 12px; } Text { text: "[ ] 2. 15時からミーティング"; font-size: 12px; } Text { text: "[V] 3. Slintアプリの基盤設計を考える"; font-size: 12px; color: #95a5a6; } }

    // ─── 【メイン画面】 ───
    export component AppWindow inherits Window {

        title: "大拡張！8大機能カスタム基盤";
        width: 470px; height: 650px; background: #f4f6f7;
        in-out property <[ComponentState]> comp-list: [];
        in property <string> current-time-str: "19:05:00";
        in property <string> current-date-str: "2026年06月03日";
        in-out property <int> selected-idx: -1; 

        callback cmd-toggle-size(int); callback cmd-move(int, bool);
        callback cmd-hide(int); callback cmd-add-next(); callback cmd-select-handle(int);

        VerticalBox {
            padding: 15px; spacing: 10px;
            HorizontalBox {
                alignment: space-between;
                Text { text: "8大機能ダッシュボード"; font-size: 16px; font-weight: 700; }
                Button { text: "+ 新機能(6,7,8)を増やす！"; primary: true; clicked => { root.cmd-add-next(); } }
            }
            ScrollView {
                VerticalBox {
                    spacing: 10px; padding: 0;
                    
                    // 先輩が一番クリアになったと納得された、最高のテーブル駆動ループ！
                    for state[idx] in root.comp-list : CustomWidgetFrame {
                        title: state.title;
                        is-expanded: state.is-expanded;
                        is-selected: root.selected-idx == idx; 

                        toggle-size => { root.cmd-toggle-size(idx); }
                        move-up => { root.cmd-move(idx, true); }
                        move-down => { root.cmd-move(idx, false); }
                        hide-component => { root.cmd-hide(idx); }
                        handle-clicked => { root.cmd-select-handle(idx); }

                        if state.comp-type == "zandaka" : ZandakaComp {}
                        if state.comp-type == "clock" : ClockComp { time: root.current-time-str; }
                        if state.comp-type == "date" : BoxComp { date: root.current-date-str; }
                        if state.comp-type == "memo" : MemoComp {}
                        if state.comp-type == "ad" : AdComp {}
                        if state.comp-type == "tenki" : TenkiComp {}
                        if state.comp-type == "kabuka" : KabukaComp {}
                        if state.comp-type == "todo" : TodoComp {}
                    }
                }
            }
            HorizontalLayout {
                alignment: start; 
                
                padding-left: 10px; 
                Button {
                    text: "About";
                    
                    width: 100px; 

                    clicked => {
                        about_popup.show();

                    }
                }
            }
        }

        about_popup := PopupWindow {
            // 💡 1. ポップアップ自体を画面全体（100%）のサイズにするにゃ！
            width: root.width;
            height: root.height;
            x: 0;
            y: 0;
            
            // 💡 2. 画面全体を覆う「ベースの透明な板」を用意するにゃ
            Rectangle {
                width: 100%;
                height: 100%;
                background: transparent; // 完全に透明にゃ

                // 💡 3. その上に、サイズを「300x200」に完全固定したグレーの板を置くにゃ！
                // 💡 4. これで左右・上下の余白が1ドットの狂いもなく【完全均等】に計算されるにゃ！
                Rectangle {
                    width: 300px;
                    height: 200px;
                    x: (parent.width - self.width) * 0.5; // 左右均等中央
                    y: (parent.height - self.height) * 0.5; // 上下均等中央
                    
                    background: #67696b5e;
                    border-radius: 4px; // 綺麗な角丸
                    
                    // 💡 5. グレーの板の「中身だけ」を綺麗に並べる箱にゃ
                    VerticalBox {
                        padding: 15px;
                        spacing: 10px; // 要素間の隙間を空けて見やすくするにゃ
                        Text {
                            color: #1b2530; font-size: 16px; font-weight: 700;
                            horizontal-alignment: center; 
                            text: "App Ver.1.0";
                        }
                        HorizontalLayout { 
                             alignment: center;
                            AboutSlint {
                                width: 200px;
                                height: 80px;
                            }
                        }
                        HorizontalLayout { 
                            alignment: center;
                            Button {
                                text: "閉じる";
                                width: 100px;
                                clicked => { about_popup.close(); }
                            }

                        }

                    }

                }
            }
        }
    }
}
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    // 💡 1〜5番目のパーツの初期登録（完璧なデータテーブル）
    let initial_components = vec![
        ComponentState { id: 1, title: "資産・リアルタイム残高".into(), comp_type: "zandaka".into(), is_expanded: true, is_visible: true },
        ComponentState { id: 2, title: "チクタク時計".into(), comp_type: "clock".into(), is_expanded: false, is_visible: true },
        ComponentState { id: 3, title: "今日の日付".into(), comp_type: "date".into(), is_expanded: false, is_visible: true },
        ComponentState { id: 4, title: "クイック・ビジネスメモ".into(), comp_type: "memo".into(), is_expanded: true, is_visible: true },
        ComponentState { id: 5, title: "スポンサー広告枠".into(), comp_type: "ad".into(), is_expanded: true, is_visible: true },
    ];
    
    let model = std::rc::Rc::new(slint::VecModel::from(initial_components));
    ui.set_comp_list(model.clone().into());

    // ① 広げる・縮めるのステート変更
    let model_clone = model.clone();
    ui.on_cmd_toggle_size(move |idx| {
        if let Some(mut data) = model_clone.row_data(idx as usize) {
            data.is_expanded = !data.is_expanded;
            model_clone.set_row_data(idx as usize, data);
        }
    });

    // ② ボタンによる上下の並び替え
    let model_clone = model.clone();
    ui.on_cmd_move(move |idx, move_up| {
        let i = idx as usize;
        let mut vec: Vec<ComponentState> = model_clone.iter().collect();
        if move_up && i > 0 { vec.swap(i, i - 1); } 
        else if !move_up && i < vec.len() - 1 { vec.swap(i, i + 1); }
        // 🛠️ 【先輩の設計】最新の配列で一括上書きリセット！
        model_clone.set_vec(vec);
    });

    // ③ ✕ボタンによる物理削除（配列から完全抹消）
    let model_clone = model.clone();
    ui.on_cmd_hide(move |idx| {
        let i = idx as usize;
        let mut vec: Vec<ComponentState> = model_clone.iter().collect();
        if i < vec.len() {
            vec.remove(i); 
            // 🛠️ 【先輩の設計】クローンから消した最新データで丸ごと全部入れ替え！
            model_clone.set_vec(vec);
        }
    });

    // ④ 新機能(6,7,8)を安全に増やすロジック
    let model_clone = model.clone();
    ui.on_cmd_add_next(move || {
        let vec: Vec<ComponentState> = model_clone.iter().collect();
        if let Some(last_item) = vec.last() {
            let last_type = last_item.comp_type.as_str();
            if last_type == "ad" {
                model_clone.push(ComponentState { id: 6, title: "6. お天気ウェザー".into(), comp_type: "tenki".into(), is_expanded: true, is_visible: true });
            } else if last_type == "tenki" {
                model_clone.push(ComponentState { id: 7, title: "7. ビジネス経済指標".into(), comp_type: "kabuka".into(), is_expanded: true, is_visible: true });
            } else if last_type == "kabuka" {
                model_clone.push(ComponentState { id: 8, title: "8. 本日のタスク(Todo)".into(), comp_type: "todo".into(), is_expanded: true, is_visible: true });
            }
        }
    });

    // ⑤ 2クリック式・位置スワップ
    let model_clone = model.clone();
    let ui_handle = ui.as_weak();
    ui.on_cmd_select_handle(move |idx| {
        if let Some(ui_instance) = ui_handle.upgrade() {
            let first_selected = ui_instance.get_selected_idx();
            if first_selected == -1 {
                ui_instance.set_selected_idx(idx);
            } else {
                let src_idx = first_selected as usize;
                let dst_idx = idx as usize;
                if src_idx != dst_idx {
                    let mut vec: Vec<ComponentState> = model_clone.iter().collect();
                    vec.swap(src_idx, dst_idx); 
                    // 🛠️ 【先輩の設計】2クリック時も最新の配列で一括全部入れ替え！
                    model_clone.set_vec(vec);
                }
                ui_instance.set_selected_idx(-1);
            }
        }
    });

    // 🕒 リアルタイムタイマー（本物のシステム時計から時・分・秒を1秒ごとに取得） [INDEX]
    let ui_timer_handle = ui.as_weak();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let now = std::time::SystemTime::now();
            let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let total_seconds = since_the_epoch.as_secs();
            
            // 日本時間（UTC+9）に補正して時分秒を算出
            let jst_seconds = total_seconds + (9 * 3600); 
            let secs = jst_seconds % 60;
            let mins = (jst_seconds / 60) % 60;
            let hours = (jst_seconds / 3600) % 24;
            let current_time_string = format!("{:02}:{:02}:{:02}", hours, mins, secs);
            
            let ui_handle_for_loop = ui_timer_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui_instance) = ui_handle_for_loop.upgrade() {
                    ui_instance.set_current_time_str(current_time_string.into());
                }
            });
        }
    });

    // 初期化設定（プロパティ名に完全に一致した正しい関数名です！） [INDEX]
    ui.set_current_time_str("00:00:00".into());
    ui.set_current_date_str("2026年06月06日".into());

    ui.run()
}
