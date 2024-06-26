let nodes = vec![
            NodeConstructor::new(
                0,
                NodeArgs {
                    outline: Some(egui::Color32::LIGHT_BLUE),
                    ..Default::default()
                },
            )
            .with_origin([50.0, 150.0].into())
            .with_title(|ui| ui.label("Example Node A"))
            .with_input_attribute(
                0,
                PinArgs {
                    shape: PinShape::Triangle,
                    ..Default::default()
                },
                |ui| ui.label("Input"),
            )
            .with_static_attribute(1, |ui| ui.label("Can't Connect to Me"))
            .with_output_attribute(
                2,
                PinArgs {
                    shape: PinShape::TriangleFilled,
                    ..Default::default()
                },
                |ui| ui.label("Output"),
            ),
            NodeConstructor::new(1, Default::default())
                .with_origin([225.0, 150.0].into())
                .with_title(|ui| ui.label("Example Node B"))
                .with_static_attribute(3, |ui| ui.label("Can't Connect to Me"))
                .with_output_attribute(4, Default::default(), |ui| ui.label("Output"))
                .with_input_attribute(5, Default::default(), |ui| ui.label("Input")),
            NodeConstructor::new(2, Default::default())
                .with_origin([225.0, 150.0].into())
                .with_title(|ui| ui.label("Example Node B"))
                .with_static_attribute(6, |ui| ui.label("Can't Connect to Me"))
                .with_output_attribute(7, Default::default(), |ui| ui.label("Output"))
                .with_input_attribute(8, Default::default(), |ui| ui.label("Input")),
        ];