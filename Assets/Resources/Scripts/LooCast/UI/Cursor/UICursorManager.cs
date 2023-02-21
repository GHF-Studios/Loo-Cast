using System;
using UnityEngine;

namespace LooCast.UI.Cursor
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UICursorManager : SubModuleManager
    {
        #region Static Properties
        public static UICursorManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[UICursorManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UICursorManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UICursorManager instance;
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
            looCastNamespace = new Namespace("Cursor", rootNamespace);
            looCastType = new Type(typeof(UICursorManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type asteroidCursorType = new Type(typeof(AsteroidCursor), looCastNamespace);
            Type missionButtonCursorType = new Type(typeof(MissionButtonCursor), looCastNamespace);

            typeManager.RegisterType(asteroidCursorType);
            typeManager.RegisterType(missionButtonCursorType);
            #endregion
        }
        #endregion
    }
}