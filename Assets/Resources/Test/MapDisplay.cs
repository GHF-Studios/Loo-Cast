using UnityEngine;

namespace LooCast.Test
{
    public class MapDisplay : MonoBehaviour
    {
        [SerializeField] private Material baseMaterial;
        [SerializeField] private Renderer textureRenderer;
        [SerializeField] private bool scalePlane;

        public void DrawTexture(Texture2D texture)
        {
            Material material = new Material(baseMaterial);
            material.mainTexture = texture;
            textureRenderer.sharedMaterial = material;
            textureRenderer.transform.localScale = new Vector3(scalePlane ? texture.width : 1, 1, scalePlane ? texture.height : 1);
        }
    } 
}
