extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn silk_sources(src: &PathBuf) -> Vec<PathBuf> {
    let mut sources = vec![
        "CNG.c", 
        "code_signs.c", 
        "init_decoder.c", 
        "decode_core.c", 
        "decode_frame.c", 
        "decode_parameters.c",
        "decode_indices.c", 
        "decode_pulses.c", 
        "decoder_set_fs.c", 
        "dec_API.c", 
        "enc_API.c", 
        "encode_indices.c", 
        "encode_pulses.c", 
        "gain_quant.c", 
        "interpolate.c", 
        "LP_variable_cutoff.c", 
        "NLSF_decode.c", 
        "NSQ.c", 
        "NSQ_del_dec.c", 
        "PLC.c", 
        "shell_coder.c", 
        "tables_gain.c", 
        "tables_LTP.c", 
        "tables_NLSF_CB_NB_MB.c", 
        "tables_NLSF_CB_WB.c", 
        "tables_other.c", 
        "tables_pitch_lag.c", 
        "tables_pulses_per_block.c", 
        "VAD.c", 
        "control_audio_bandwidth.c", 
        "quant_LTP_gains.c", 
        "VQ_WMat_EC.c", 
        "HP_variable_cutoff.c", 
        "NLSF_encode.c", 
        "NLSF_VQ.c", 
        "NLSF_unpack.c", 
        "NLSF_del_dec_quant.c", 
        "process_NLSFs.c", 
        "stereo_LR_to_MS.c", 
        "stereo_MS_to_LR.c", 
        "check_control_input.c", 
        "control_SNR.c", 
        "init_encoder.c", 
        "control_codec.c", 
        "A2NLSF.c", 
        "ana_filt_bank_1.c", 
        "biquad_alt.c", 
        "bwexpander_32.c", 
        "bwexpander.c", 
        "debug.c", 
        "decode_pitch.c", 
        "inner_prod_aligned.c", 
        "lin2log.c", 
        "log2lin.c", 
        "LPC_analysis_filter.c", 
        "LPC_inv_pred_gain.c", 
        "table_LSF_cos.c", 
        "NLSF2A.c", 
        "NLSF_stabilize.c", 
        "NLSF_VQ_weights_laroia.c", 
        "pitch_est_tables.c", 
        "resampler.c", 
        "resampler_down2_3.c", 
        "resampler_down2.c", 
        "resampler_private_AR2.c", 
        "resampler_private_down_FIR.c", 
        "resampler_private_IIR_FIR.c", 
        "resampler_private_up2_HQ.c", 
        "resampler_rom.c", 
        "sigm_Q15.c", 
        "sort.c", 
        "sum_sqr_shift.c", 
        "stereo_decode_pred.c", 
        "stereo_encode_pred.c", 
        "stereo_find_predictor.c", 
        "stereo_quant_pred.c", 
        "LPC_fit.c",
        "float/apply_sine_window_FLP.c",
        "float/corrMatrix_FLP.c",
        "float/encode_frame_FLP.c",
        "float/find_LPC_FLP.c",
        "float/find_LTP_FLP.c",
        "float/find_pitch_lags_FLP.c",
        "float/find_pred_coefs_FLP.c",
        "float/LPC_analysis_filter_FLP.c",
        "float/LTP_analysis_filter_FLP.c",
        "float/LTP_scale_ctrl_FLP.c",
        "float/noise_shape_analysis_FLP.c",
        "float/process_gains_FLP.c",
        "float/regularize_correlations_FLP.c",
        "float/residual_energy_FLP.c",
        "float/warped_autocorrelation_FLP.c",
        "float/wrappers_FLP.c",
        "float/autocorrelation_FLP.c",
        "float/burg_modified_FLP.c",
        "float/bwexpander_FLP.c",
        "float/energy_FLP.c",
        "float/inner_product_FLP.c",
        "float/k2a_FLP.c",
        "float/LPC_inv_pred_gain_FLP.c",
        "float/pitch_analysis_core_FLP.c",
        "float/scale_copy_vector_FLP.c",
        "float/scale_vector_FLP.c",
        "float/schur_FLP.c",
        "float/sort_FLP.c",
    ];
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse4.1") {
            sources.extend(vec![
                "x86/NSQ_sse4_1.c",
                "x86/NSQ_del_dec_sse4_1.c",
                "x86/x86_silk_map.c",
                "x86/VAD_sse4_1.c",
                "x86/VQ_WMat_EC_sse4_1.c",
            ]);
        }
    }
    sources.iter().map(|path| PathBuf::from(src).join("silk").join(path)).collect()
}

fn celt_sources(src: &PathBuf) -> Vec<PathBuf> {
    let mut sources = vec![
        "bands.c",
        "celt.c",
        "celt_encoder.c",
        "celt_decoder.c",
        "cwrs.c",
        "entcode.c",
        "entdec.c",
        "entenc.c",
        "kiss_fft.c",
        "laplace.c",
        "mathops.c",
        "mdct.c",
        "modes.c",
        "pitch.c",
        "celt_lpc.c",
        "quant_bands.c",
        "rate.c",
        "vq.c",
    ];
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse") {
            sources.extend(vec![
                "x86/x86cpu.c",
                "x86/x86_celt_map.c",
                "x86/pitch_sse.c",
            ]);
        }
        if is_x86_feature_detected!("sse2") {
            sources.extend(vec![
                "x86/pitch_sse2.c",
                "x86/vq_sse2.c",
            ]);
        }
        if is_x86_feature_detected!("sse4.1") {
            sources.extend(vec![
                "x86/pitch_sse4_1.c",
            ]);
        }
    }
    #[cfg(target_arch = "arm")]
    {
        sources.extend(vec![
            "arm/armcpu.c",
            "arm/arm_celt_map.c",
        ]);
    }
    sources.iter().map(|path| PathBuf::from(src).join("celt").join(path)).collect()
}

fn opus_sources(src: &PathBuf) -> Vec<PathBuf> {
    let sources = vec![
        "opus.c",
        "opus_decoder.c",
        "opus_encoder.c",
        "opus_multistream.c",
        "opus_multistream_encoder.c",
        "opus_multistream_decoder.c",
        "repacketizer.c",
        "analysis.c",
        "mlp.c",
        "mlp_data.c",
    ];
    sources.iter().map(|path| PathBuf::from(src).join("src").join(path)).collect()
}

fn build(src: &PathBuf, dst: &PathBuf) {
    let mut cc = cc::Build::new();
    cc.include(PathBuf::from(src).join("include"));
    cc.include(PathBuf::from(src).join("silk"));
    cc.include(PathBuf::from(src).join("silk/float"));
    cc.include(PathBuf::from(src).join("src"));
    cc.include(PathBuf::from(src).join("celt"));
    cc.include(PathBuf::from(src));
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        cc.include(PathBuf::from(src).join("silk/x86"));
        if is_x86_feature_detected!("sse") { cc.flag("-msse"); }
        if is_x86_feature_detected!("sse2") { cc.flag("-msse2"); }
        if is_x86_feature_detected!("sse4.1") { cc.flag("-msse4.1"); }
    }
    #[cfg(target_arch = "arm")]
    {
        cc.include(PathBuf::from(src).join("silk/arm"));
    }
    cc.files(celt_sources(src));
    cc.files(silk_sources(src));
    cc.files(opus_sources(src));
    cc.define("USE_ALLOCA", "ON");
    cc.define("OPUS_BUILD", "ON");
    cc.define("HAVE_LRINTF", "ON");
    cc.static_flag(false);
    cc.out_dir(dst);
    cc.opt_level(3);
    cc.compile("libopus.a");

    println!("cargo:rustc-flags=-l static=opus");
    println!("cargo:rustc-flags=-L {}", dst.display());
    println!("cargo:rerun-if-changed={}", src.to_str().unwrap());
}

fn main() {
    let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("opus");
    let dst = PathBuf::from(&env::var_os("OUT_DIR").unwrap());
    build(&src, &dst);
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
