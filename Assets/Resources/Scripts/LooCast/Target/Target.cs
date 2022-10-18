using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Target
{
    using Health;

    public class Target : IEquatable<Target>
    {
        public GameObject GameObject { get { return collider.gameObject; } }
        public Transform Transform { get { return collider.gameObject.transform; } }
        public IHealth Health { get { return health; } }
        public Collider2D Collider { get { return collider; } }
        public UnityEvent OnInvalidated { get { return onInvalidated; } }

        private IHealth health;
        private Collider2D collider;
        private UnityEvent onInvalidated;

        public Target(IHealth health, Collider2D collider)
        {
            this.health = health;
            this.collider = collider;
            onInvalidated = new UnityEvent();
            health.OnKilled.AddListener(() => { onInvalidated.Invoke(); });
        }

        public bool Equals(Target other)
        {
            return collider.Equals(other.collider);
        }
    }
}
