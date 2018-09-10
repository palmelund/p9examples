using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class BusyPlay : MonoBehaviour
{
    // Update is called once per frame
    void Start()
    {
        var audio = GetComponent<AudioSource>();
        audio.Play();
        Debug.Log(audio.isPlaying);
    }
}