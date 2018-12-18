using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using UnityEngine;
using UnityEngine.UI;

public class UiTest : MonoBehaviour
{
	public Text Text;
	public InputField Input;
	
	// Update is called once per frame
	void Start () {
		// Text Field
		Debug.Log("S01: " + Text.text);
		
		Task.Run(() => Debug.Log("S02: " + Text.text));
		new Thread(() => Debug.Log("S03: " + Text.text)).Start();

		Text.text = "S04";
		Task.Run(() => Text.text = "S05");
		new Thread(() => Text.text = "S06").Start();
		
		// Input Field
		Debug.Log("S07: " + Input.text);
		
		Task.Run(() => Debug.Log("S08: " + Input.text));
		new Thread(() => Debug.Log("S09: " + Input.text)).Start();

		Input.text = "S10";
		Task.Run(() => Input.text = "S11");
		new Thread(() => Input.text = "S12").Start();
	}
}
