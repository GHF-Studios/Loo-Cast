using System;
using UnityEngine;

namespace LooCast.Player
{
    public class PlayerManager : ModuleManager
    {
        #region Static Properties
        public static PlayerManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[PlayerManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<PlayerManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static PlayerManager instance;
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
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Player", rootNamespace);
            looCastType = new Type(typeof(PlayerManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type playerType = new Type(typeof(Player), looCastNamespace);

            typeManager.RegisterType(playerType);
            #endregion
        }
        #endregion
    }
}