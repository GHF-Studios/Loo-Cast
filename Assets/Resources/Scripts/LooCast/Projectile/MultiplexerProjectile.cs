using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Projectile
{
    using Health;
    using Sound;
    using Target;
    using Random;
    using LooCast.Util;

    public class MultiplexerProjectile : Projectile
    {
        public int fragments { get; protected set; }
        public int fragmentArmorPenetration { get; protected set; }
        public bool followTarget { get; protected set; }
        protected GameObject fragmentPrefab;
        protected GameSoundHandler soundHandler;

        public virtual void Initialize(NewTarget target, GameObject origin, IHealth.TeamType team, float damage, float critChance, float critDamage, float knockback, float speed, float size, float lifetime, int piercing, int armorPenetration, int fragments, int fragmentArmorPenetration, bool followTarget, GameObject fragmentPrefab)
        {
            base.Initialize(target, origin, team, damage, critChance, critDamage, knockback, speed, size, lifetime, piercing, armorPenetration);
            this.fragmentPrefab = fragmentPrefab;
            soundHandler = FindObjectOfType<GameSoundHandler>();
            this.fragments = fragments;
            this.fragmentArmorPenetration = fragmentArmorPenetration;
            this.followTarget = followTarget;

            if (target == null || target.Transform == null)
            {
                float x = Random.Range(-1f, 1f);
                float y = Random.Range(-1f, 1f);
                Vector3 direction = new Vector3(x, y, 0f).normalized;
                rb.velocity = direction;
            }
            else
            {
                float projectileArrivalTime = (target.Transform.position - origin.transform.position).magnitude / speed;
                Vector3 targetVelocity = target.GameObject.GetComponent<Rigidbody2D>().velocity;
                targetVelocity.z = 0;
                Vector3 estimatedProjectileHitPos = target.Transform.position + targetVelocity * projectileArrivalTime;

                rb.velocity = (estimatedProjectileHitPos - transform.position).normalized;
            }
            rb.velocity *= speed;

            float angle = Mathf.Atan2(rb.velocity.y, rb.velocity.x) * Mathf.Rad2Deg - 90.0f;
            transform.rotation = Quaternion.AngleAxis(angle, Vector3.forward);
        }

        public virtual void Kill(Collider2D collision)
        {
            base.Kill();

            for (int i = 0; i < fragments; i++)
            {
                GameObject bulletObject = Instantiate(fragmentPrefab, transform.position, Quaternion.identity);
                bulletObject.transform.position += new Vector3(0, 0, 0.1f);
                bulletObject.GetComponent<MultiplexerFragmentProjectile>().Initialize(Origin, Team, collision, Damage, CritChance, CritDamage, Knockback, Speed * 5.0f, Size * 0.5f, 0.5f, Piercing, fragmentArmorPenetration);
            }
            soundHandler.SoundShoot();
        }

        private void OnTriggerEnter2D(Collider2D collision)
        {
            bool CheckTags(params string[] tags)
            {
                foreach (string tag in tags)
                {
                    if (collision.gameObject.CompareTag(tag))
                    {
                        return true;
                    }
                }
                return false;
            }

            if (CheckTags(TeamUtil.GetEnemyTags(Team)))
            {
                if (Pierced > Piercing)
                {
                    Kill(collision);
                    return;
                }
                
                Pierced += 1;
                IHealth collisionHealth = collision.gameObject.GetComponentInParent<IHealth>();
                collisionHealth.Damage(new DamageInfo(Origin, gameObject, Damage * Random.Range(2.5f, 5.0f), Knockback, ArmorPenetration, CritChance, CritDamage));


                if (Pierced > Piercing)
                {
                    Kill(collision);
                    return;
                }
            }
        }
    } 
}
