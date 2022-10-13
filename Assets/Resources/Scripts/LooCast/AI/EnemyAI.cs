using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.AI
{
    using Core;
    using Random;
    using Movement;
    using Player;

    public class EnemyAI : ExtendedMonoBehaviour
    {
        #region Enums
        private enum State
        {
            Roaming,
            ChaseTarget
        }
        #endregion

        private const float roamingPositionMinDistance = 5.0f;
        private const float detectionRange = 50.0f;

        private IMovement movement;
        private Vector3 startingPosition;
        private Vector3 roamingPosition;
        private Player player;
        private State state;
        private Variable.Multiplier currentSpeedMultiplier;

        private void Awake()
        {
            movement = GetComponent<IMovement>();
            player = FindObjectOfType<Player>();
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
            switch (state)
            {
                case State.Roaming:
                    movement.AccelerateToPosition(roamingPosition);
                    if (Vector3.Distance(transform.position, roamingPosition) < roamingPositionMinDistance)
                    {
                        roamingPosition = GetRoamingPosition();
                    }

                    FindTarget();
                    break;
                case State.ChaseTarget:
                    movement.AccelerateToPosition(player.transform.position);
                    break;
            }
        }

        private Vector3 GetRoamingPosition()
        {
            return startingPosition + Random.Direction() * Random.Range(10.0f, 25.0f);
        }

        private void FindTarget()
        {
            if (Vector3.Distance(transform.position, player.transform.position) < detectionRange)
            {
                state = State.ChaseTarget;
                movement.Speed.RemovePermanentMultiplier(currentSpeedMultiplier);
            }
        }
    } 
}
