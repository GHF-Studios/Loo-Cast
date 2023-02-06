using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Projectile
{
    using Health;
    using LooCast.Util;
    using Random;

    public class MultiplexerFragmentProjectile : Projectile
    {
        public virtual void Initialize(GameObject origin, IHealth.TeamType team, Collider2D ignoreCollider, float damage, float critChance, float critDamage, float knockback, float speed, float size, float lifetime, int piercing, int armorPenetration)
        {
            base.Initialize(null, origin, team, damage, critChance, critDamage, knockback, speed, size, lifetime, piercing, armorPenetration);

            Physics2D.IgnoreCollision(GetComponent<Collider2D>(), ignoreCollider);

            float x = Random.Range(-1f, 1f);
            float y = Random.Range(-1f, 1f);
            Vector3 direction = new Vector3(x, y, 0f).normalized;
            rb.velocity = direction * speed;

            float angle = Mathf.Atan2(rb.velocity.y, rb.velocity.x) * Mathf.Rad2Deg - 90.0f;
            transform.rotation = Quaternion.AngleAxis(angle, Vector3.forward);
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
                    Kill();
                    return;
                }

                Pierced += 1;
                IHealth collisionHealth = collision.gameObject.GetComponentInParent<IHealth>();
                collisionHealth.Damage(new DamageInfo(Origin, gameObject, Damage * Random.Range(2.5f, 5.0f), Knockback, ArmorPenetration, CritChance, CritDamage));

                if (Pierced > Piercing)
                {
                    Kill();
                    return;
                }
            }
        }
    } 
}
