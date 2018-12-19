using System.Collections;
using System.Collections.Generic;
using System.Threading;
using UnityEngine;

public class InputTester : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        if(Input.GetKeyDown(KeyCode.Space))
        {
            Debug.Log("Down");
        }

        if (Input.GetKey(KeyCode.Space))
        {
            Debug.Log("Pressed");
        }

        if (Input.GetKeyUp(KeyCode.Space))
        {
            Debug.Log("Up");
        }
        
        Thread.Sleep(1000);
    }
}
