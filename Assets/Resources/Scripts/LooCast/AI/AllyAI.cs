using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.AI
{
    using Core;
    using Random;
    using Movement;

    public class AllyAI : ExtendedMonoBehaviour
    {
        #region Classes
        
        #endregion

        #region Enums
        private enum State
        {
            Roaming,
            Evade
        }
        #endregion

        private const float roamingPositionMinDistance = 5.0f;
        private const float detectionRange = 50.0f;

        [SerializeField] private LayerMask enemyLayerMask;

        private IMovement movement;
        private Vector3 startingPosition;
        private Vector3 roamingPosition;
        private Vector3 evadeDirection;
        private State state;
        private Variable.Multiplier currentSpeedMultiplier;

        private void Awake()
        {
            movement = GetComponent<IMovement>();
        }

        private void Start()
        {
            startingPosition = transform.position;
            roamingPosition = GetRoamingPosition();
            state = State.Roaming;
            currentSpeedMultiplier = movement.Speed.AddPermanentMultiplier(0.15f);
        }

        protected override void PauseableUpdate()
        {
            FindEnemies();

            switch (state)
            {
                case State.Roaming:
                    movement.AccelerateToPosition(roamingPosition);
                    if (Vector3.Distance(transform.position, roamingPosition) < roamingPositionMinDistance)
                    {
                        roamingPosition = GetRoamingPosition();
                    }
                    break;
                case State.Evade:
                    movement.AccelerateInDirection(evadeDirection);
                    break;
            }
        }

        private Vector3 GetRoamingPosition()
        {
            return startingPosition + Random.Direction() * Random.Range(10.0f, 25.0f);
        }

        private void FindEnemies()
        {
            Collider2D[] enemyColliders = Physics2D.OverlapCircleAll(transform.position, detectionRange, enemyLayerMask);
            if (enemyColliders.Length > 0)
            {
                List<Vector3> evadeDirectionList = new List<Vector3>();
                foreach (Collider2D enemyCollider in enemyColliders)
                {
                    float evadeDirectionWeight = Mathf.Pow(detectionRange / Vector2.Distance(transform.position, enemyCollider.transform.position), 2);
                    Vector3 weightedEvadeDirection = (transform.position - enemyCollider.transform.position).normalized * evadeDirectionWeight;
                    evadeDirectionList.Add(weightedEvadeDirection);
                }
                evadeDirection = evadeDirectionList.Aggregate(new Vector3(0, 0, 0), (sumVector, currentVector) => sumVector + currentVector) / evadeDirectionList.Count;
                state = State.Evade;
                movement.Speed.RemovePermanentMultiplier(currentSpeedMultiplier);
            }
            else
            {
                state = State.Roaming;
                currentSpeedMultiplier = movement.Speed.AddPermanentMultiplier(0.15f);

                startingPosition = transform.position;
                roamingPosition = GetRoamingPosition();
            }
        }
    } 
}
