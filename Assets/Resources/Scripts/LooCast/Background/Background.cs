using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Background
{
    using Core;

    public class Background : ExtendedMonoBehaviour
    {
        [SerializeField] private Sprite backgroundSprite;
        [SerializeField] private SpriteRenderer[] backgroundRenderers;

        private void Start()
        {
            for (int i = 0; i < backgroundRenderers.Length; i++)
            {
                backgroundRenderers[i].sprite = backgroundSprite;
            }
        }

        protected override void PauseableUpdate()
        {
            Vector3 cameraPos = Camera.main.transform.position;
            Vector2 shift = Vector2.zero;
            if (cameraPos.x > transform.position.x + (backgroundSprite.bounds.size.x/2))
            {
                shift.x = 1;
            }
            else if (cameraPos.x < transform.position.x - (backgroundSprite.bounds.size.x / 2))
            {
                shift.x = -1;
            }
            if (cameraPos.y > transform.position.y + (backgroundSprite.bounds.size.y / 2))
            {
                shift.y = 1;
            }
            else if (cameraPos.y < transform.position.y - (backgroundSprite.bounds.size.y / 2))
            {
                shift.y = -1;
            }

            shift.x *= backgroundSprite.bounds.size.x;
            shift.y *= backgroundSprite.bounds.size.y;

            if (shift.x != 0 || shift.y != 0)
            {
                transform.position = new Vector3(transform.position.x + shift.x, transform.position.y + shift.y, transform.position.z);
            }
        }
    } 
}
