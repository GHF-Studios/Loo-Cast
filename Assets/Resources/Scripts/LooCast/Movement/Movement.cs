using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using Core;
    using Data;
    using Stat;
    using Util;

    [RequireComponent(typeof(Rigidbody2D), typeof(Collider2D))]
    public abstract class Movement : Component
    {
        public UnityEvent OnMovementEnabled { get; protected set; }
        public UnityEvent OnMovementDisabled { get; protected set; }

        public FloatStat Speed { get; protected set; }

        public Rigidbody2D Rigidbody { get; protected set; }
        public Collider2D Collider { get; protected set; }

        private Vector3 PAUSE_currentVelocity;

        protected void Initialize(MovementData data)
        {
            OnMovementEnabled = new UnityEvent();
            OnMovementDisabled = new UnityEvent();

            Speed = new FloatStat(data.BaseSpeed.Value);
            Speed.AddPermanentMultiplier(Constants.INERTIAL_COEFFICIENT);

            Rigidbody = GetComponent<Rigidbody2D>();
            Collider = GetComponent<Collider2D>();
        }

        protected override void OnPauseableFixedUpdate()
        {
            Accelerate();
        }

        public abstract void Accelerate();

        protected override void OnPause()
        {
            PAUSE_currentVelocity = Rigidbody.velocity;
            Rigidbody.velocity = Vector3.zero;
        }

        protected override void OnResume()
        {
            Rigidbody.velocity = PAUSE_currentVelocity;
            PAUSE_currentVelocity = Vector3.zero;
        }
    } 
}
