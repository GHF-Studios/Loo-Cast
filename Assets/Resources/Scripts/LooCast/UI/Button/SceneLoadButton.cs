using UnityEngine;

namespace LooCast.UI.Button
{
    using Core;

    public class SceneLoadButton : Button
    {
        [SerializeField] private MainManager.SceneType sceneType;

        private void Start()
        {
            Initialize();
        }

        public override void OnClick()
        {
            MainManager.LoadScene(sceneType);
        }
    }
}
