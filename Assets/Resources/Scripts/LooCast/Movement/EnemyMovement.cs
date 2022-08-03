using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Movement
{
    using Data;
    using Target;

    public class EnemyMovement : TargetedMovement
    {
        public EnemyMovementData Data;
        private GameObject playerObject;
        private CircleCollider2D playerCollider;
        private bool isMovementEnabled;

        private void Start()
        {
            Initialize(Data);

            isMovementEnabled = Data.IsMovementEnabled.Value;

            playerObject = GameObject.FindGameObjectWithTag("Player");
            playerCollider = playerObject.GetComponent<CircleCollider2D>();

            SetTarget(new Target(playerCollider));
            Speed.AddPermanentMultiplier(UnityEngine.Random.Range(0.9f, 1.1f));
        }

        public override void Accelerate()
        {
            if (isMovementEnabled)
            {
                Rigidbody.AddForce((target.transform.position - transform.position).normalized * Speed.Value);

                Vector2 lookDir = target.transform.position - transform.position;
                float angle = Mathf.Atan2(lookDir.y, lookDir.x) * Mathf.Rad2Deg - 90.0f;
                transform.rotation = Quaternion.Euler(transform.rotation.eulerAngles.x, transform.rotation.eulerAngles.y, angle);
            }
        }
    } 
}
