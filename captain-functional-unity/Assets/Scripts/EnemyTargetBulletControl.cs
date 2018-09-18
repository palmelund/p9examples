using UnityEngine;

public class EnemyTargetBulletControl : MonoBehaviour
{
    public float Speed;

    private static GameObject _player;

    private void Awake()
    {
        if (_player == null)
        {
            _player = GameObject.FindGameObjectWithTag("Player");   
        }
    }

    private void Update()
    {
        if (_player)
        {
            transform.position = Vector3.MoveTowards(transform.position, _player.transform.position, Time.deltaTime * Speed);   
            
        }
    }

    private void OnCollisionEnter(Collision other)
    {
        if (other.gameObject.tag.Equals("Player"))
        {
            _player.GetComponent<PlayerControl>().Health--;
            Destroy(gameObject);
        }
    }
}