using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;
using UnityScript.Core;

public class PlayerControl : MonoBehaviour
{
    public float Speed;
    public List<GameObject> Bullets;
    [SerializeField]
    private int _health = 3;

    public float FireRate;

    private float _lastShot;

    private UIManager _uiManager;

    public UnityEvent Event;

    private Vector3 _V3Top;
    private Vector3 _V3Buttom;

    public int Health
    {
        get { return _health; }
        set
        {
            _health = value;
            _uiManager.UpdateHealth(Health);
            if (_health <= 0)
            {
                Event.Invoke();
                Destroy(gameObject);
            }
        }
    }

    // Use this for initialization
    void Awake()
    {
        _uiManager = GameObject.FindObjectOfType<UIManager>();
        _uiManager.UpdateHealth(Health);
        _V3Top = Camera.main.ViewportToWorldPoint(new Vector3(0.1f, 0.1f, 150.0f));
        _V3Buttom = Camera.main.ViewportToWorldPoint(new Vector3(0.9f, 0.9f, 150.0f));
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetButton("MoveUp"))
        {
            transform.position += Vector3.up * Time.deltaTime * Speed;
        }
        else if (Input.GetButton("MoveDown"))
        {
            transform.position += Vector3.down * Time.deltaTime * Speed;
        }

        if (Input.GetButton("MoveLeft"))
        {
            transform.position += Vector3.left * Time.deltaTime * Speed;
        }
        else if (Input.GetButton("MoveRight"))
        {
            transform.position += Vector3.right * Time.deltaTime * Speed;
        }

        if (Input.GetButton("Shoot") && Time.time > _lastShot + FireRate)
        {
            _lastShot = Time.time;
            Instantiate(Bullets[Random.Range(0, Bullets.Count)], transform.position, Quaternion.identity);
        }

        transform.position = new Vector3(
            Mathf.Clamp(transform.position.x, _V3Top.x, _V3Buttom.x),
            Mathf.Clamp(transform.position.y, _V3Top.y, _V3Buttom.y),
            transform.position.z);
    }

    private void OnCollisionEnter(Collision other)
    {
        if (other.gameObject.tag.Equals("Enemy"))
        {
            Health--;
            Destroy(other.gameObject);
        }
    }
}