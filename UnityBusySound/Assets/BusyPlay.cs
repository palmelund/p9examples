using System.Collections;
using System.Collections.Generic;
using System.Threading;
using UnityEngine;

public class BusyPlay : MonoBehaviour
{
    public int Iteration = 0;
    public int IterationLimit;
    
    // Update is called once per frame
    void Start()
    {
        var audio = GetComponent<AudioSource>();
        audio.Play();
    }

    private void Update()
    {
        Iteration++;

        if (Iteration == IterationLimit)
        {
            Thread.Sleep(5000);
        }
    }
}