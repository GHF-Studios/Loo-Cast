using System;
using UnityEngine;

namespace LooCast.UI.Canvas
{
    using LooCast.System;
    using LooCast.System.Management;
    
    public class UICanvasManager : SubModuleManager
    {
        #region Static Properties
        public static UICanvasManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[UICanvasManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UICanvasManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UICanvasManager instance;
        #endregion

        #region Fields

        #endregion

        #region Methods
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.UI");
            looCastNamespace = new Namespace("Canvas", rootNamespace);
            looCastType = new Type(typeof(UICanvasManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type canvasType = new Type(typeof(Canvas), looCastNamespace);
            Type gameCanvasType = new Type(typeof(GameCanvas), looCastNamespace);
            Type interfaceCanvasType = new Type(typeof(InterfaceCanvas), looCastNamespace);
            Type mainMenuCanvasType = new Type(typeof(MainMenuCanvas), looCastNamespace);
            Type screenSpaceCameraCanvasType = new Type(typeof(ScreenSpaceCameraCanvas), looCastNamespace);
            Type screenSpaceOverlayCanvasType = new Type(typeof(ScreenSpaceOverlayCanvas), looCastNamespace);
            Type worldSpaceCanvasType = new Type(typeof(WorldSpaceCanvas), looCastNamespace);

            typeManager.RegisterType(canvasType);
            typeManager.RegisterType(gameCanvasType);
            typeManager.RegisterType(interfaceCanvasType);
            typeManager.RegisterType(mainMenuCanvasType);
            typeManager.RegisterType(screenSpaceCameraCanvasType);
            typeManager.RegisterType(screenSpaceOverlayCanvasType);
            typeManager.RegisterType(worldSpaceCanvasType);
            #endregion
        }
        #endregion
    }
}