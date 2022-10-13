using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.AI
{
    using Core;
    using Random;
    using Movement;
    using Player;
    using StateMachine;

    public class EnemyAI : ExtendedMonoBehaviour
    {
        public enum State
        {
            Roaming,
            ChaseTarget
        }

        public class Roaming : State<State>
        {
            private EnemyAI enemyAI;
            private Variable.Multiplier speedMultiplier;
            private Vector3 startingPosition;
            private Vector3 roamingPosition;

            public Roaming(EnemyAI enemyAI) : base(State.Roaming)
            {
                this.enemyAI = enemyAI;
            }

            public override void Enter()
            {
                speedMultiplier = enemyAI.movement.Speed.AddPermanentMultiplier(0.15f);
                startingPosition = enemyAI.transform.position;
                roamingPosition = GetRoamingPosition();
            }

            public override void Exit()
            {
                enemyAI.movement.Speed.RemovePermanentMultiplier(speedMultiplier);
            }

            public override void PauseableUpdate()
            {
                enemyAI.movement.AccelerateToPosition(roamingPosition);
                if (Vector3.Distance(enemyAI.transform.position, roamingPosition) < enemyAI.roamingReachedDestinationDistance)
                {
                    roamingPosition = GetRoamingPosition();
                }

                if (Vector3.Distance(enemyAI.transform.position, enemyAI.player.transform.position) < enemyAI.detectionRange)
                {
                    enemyAI.finiteStateMachine.SetCurrentState(State.ChaseTarget);
                }
            }

            private Vector3 GetRoamingPosition()
            {
                return startingPosition + Random.Direction() * Random.Range(enemyAI.minRoamingDistance, enemyAI.maxRoamingDistance);
            }
        }

        public class ChaseTarget : State<State>
        {
            private EnemyAI enemyAI;

            public ChaseTarget(EnemyAI enemyAI) : base(State.ChaseTarget)
            {
                this.enemyAI = enemyAI;
            }

            public override void Update()
            {
                enemyAI.movement.AccelerateToPosition(enemyAI.player.transform.position);
            }
        }

        [SerializeField] private float minRoamingDistance;
        [SerializeField] private float maxRoamingDistance;
        [SerializeField] private float roamingReachedDestinationDistance;
        [SerializeField] private float detectionRange;
        [SerializeField] private LayerMask enemyLayerMask;

        private FiniteStateMachine<State> finiteStateMachine = new FiniteStateMachine<State>();
        private IMovement movement;
        private Player player;

        private void Awake()
        {
            movement = GetComponent<IMovement>();
            player = FindObjectOfType<Player>();
        }

        private void Start()
        {
            finiteStateMachine.Add(new Roaming(this));
            finiteStateMachine.Add(new ChaseTarget(this));

            finiteStateMachine.SetCurrentState(State.Roaming);
        }

        private void Update()
        {
            finiteStateMachine.Update();
        }

        private void FixedUpdate()
        {
            finiteStateMachine.FixedUpdate();
        }

        protected override void PauseableUpdate()
        {
            finiteStateMachine.PauseableUpdate();
        }

        protected override void PauseableFixedUpdate()
        {
            finiteStateMachine.PauseableFixedUpdate();
        }
    } 
}
