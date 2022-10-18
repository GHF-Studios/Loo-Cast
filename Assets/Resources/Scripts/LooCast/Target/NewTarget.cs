using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Target
{
    using Health;

    public class NewTarget : IEquatable<NewTarget>
    {
        public GameObject GameObject { get { return collider.gameObject; } }
        public Transform Transform { get { return collider.gameObject.transform; } }
        public IHealth Health { get { return health; } }
        public Collider2D Collider { get { return collider; } }
        public UnityEvent OnInvalidated { get { return onInvalidated; } }

        private IHealth health;
        private Collider2D collider;
        private UnityEvent onInvalidated;

        public NewTarget(IHealth health, Collider2D collider)
        {
            this.health = health;
            this.collider = collider;
            onInvalidated = new UnityEvent();
            health.OnKilled.AddListener(() => { onInvalidated.Invoke(); });
        }

        public bool Equals(NewTarget other)
        {
            return collider.Equals(other.collider);
        }
    }
}
