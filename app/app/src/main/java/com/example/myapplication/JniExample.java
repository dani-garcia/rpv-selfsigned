package com.example.myapplication;

import android.app.Application;

public class JniExample {
    public static native String networkTest(Application context);

    static {
        System.loadLibrary("rpv_selfsigned");
    }
}