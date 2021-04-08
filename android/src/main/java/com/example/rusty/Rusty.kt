package com.example.rusty

import com.facebook.react.bridge.Promise
import com.facebook.react.bridge.ReactApplicationContext
import com.facebook.react.bridge.ReactContextBaseJavaModule
import com.facebook.react.bridge.ReactMethod

class Rusty(context: ReactApplicationContext) : ReactContextBaseJavaModule(context) {

    private external fun hello(name: String): String

    companion object {
        init {
            System.loadLibrary("rust")
        }
    }

    override fun getName(): String {
        return "Rusty"
    }

    @ReactMethod
    fun hello(name: String, promise: Promise) {
        val response = hello(name)
        promise.resolve(response)
    }
}
