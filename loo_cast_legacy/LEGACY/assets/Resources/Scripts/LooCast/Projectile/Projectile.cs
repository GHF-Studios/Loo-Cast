using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Projectile
{
    using LooCast.Core;
    using LooCast.System;
    using LooCast.Health;
    using Target;

    [RequireComponent(typeof(Rigidbody2D))]
    public abstract class Projectile : ExtendedMonoBehaviour
    {
        public readonly static List<Projectile> projectiles = new List<Projectile>();

        protected Rigidbody2D rb;
        protected Target target;
        public GameObject Origin { get; protected set; }
        public IHealth.TeamType Team { get; protected set; }

        public float Damage { get; protected set; }
        public float CritChance { get; protected set; }
        public float CritDamage { get; protected set; }
        public float Knockback { get; protected set; }
        public float Speed { get; protected set; }
        public float Size { get; protected set; }
        public float Lifetime { get; protected set; }
        public int Piercing { get; protected set; }
        public int Pierced { get; protected set; }
        public int ArmorPenetration { get; protected set; }

        protected bool isAlive = true;

        private Vector3 PAUSE_currentVelocity;

        protected virtual void Initialize(Target target, GameObject origin, IHealth.TeamType team, float damage, float critChance, float critDamage, float knockback, float speed, float size, float lifetime, int piercing, int armorPenetration)
        {
            projectiles.Add(this);

            rb = GetComponent<Rigidbody2D>();
            this.target = target;
            this.Origin = origin;
            this.Team = team;
            this.Damage = damage;
            this.CritChance = critChance;
            this.CritDamage = critDamage;
            this.Knockback = knockback;
            this.Speed = speed;
            this.Size = size;
            transform.localScale *= size;
            this.Lifetime = lifetime;
            this.Piercing = piercing;
            this.Pierced = 0;
            this.ArmorPenetration = armorPenetration;

            target.Health.OnKilled.AddListener(() => { Kill(); });
        }

        protected override void PauseableUpdate()
        {
            Lifetime -= Time.deltaTime;
            if (Lifetime <= 0)
            {
                Kill();
            }
        }

        protected override void OnPause()
        {
            PAUSE_currentVelocity = rb.velocity;
            rb.velocity = Vector3.zero;
        }

        protected override void OnResume()
        {
            rb.velocity = PAUSE_currentVelocity;
            PAUSE_currentVelocity = Vector3.zero;
        }

        protected virtual void Kill()
        {
            if (isAlive)
            {
                isAlive = false;
                projectiles.Remove(this);
                Destroy(gameObject);
            }
        }
    } 
}
