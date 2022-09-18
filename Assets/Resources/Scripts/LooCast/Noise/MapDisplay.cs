using UnityEngine;

namespace LooCast.Noise
{
    public class MapDisplay : MonoBehaviour
    {
        [SerializeField] private Renderer textureRenderer;
        [SerializeField] private bool scalePlane;

        public void DrawTexture(Texture2D texture)
        {
            textureRenderer.sharedMaterial.mainTexture = texture;
            textureRenderer.transform.localScale = new Vector3(scalePlane ? texture.width : 1, 1, scalePlane ? texture.height : 1);
        }
    } 
}
