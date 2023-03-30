using System;
using UnityEngine;

namespace LooCast.Movement
{
    using LooCast.System;
    using LooCast.System.Managers;
    using LooCast.Movement.Effect;

    public class MovementManager : ModuleManager
    {
        #region Static Properties
        public static MovementManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[MovementManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<MovementManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static MovementManager instance;
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

            INamespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Movement", rootNamespace);
            Namespace effectNamespace = new Namespace("Effect", rootNamespace);
            looCastType = new Type(typeof(MovementManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type iMovementType = new Type(typeof(IMovement), looCastNamespace);
            Type allyMovementType = new Type(typeof(AllyMovement), looCastNamespace);
            Type enemyMovementType = new Type(typeof(EnemyMovement), looCastNamespace);
            Type playerMovementType = new Type(typeof(PlayerMovement), looCastNamespace);
            Type freezeMovementEffectType = new Type(typeof(FreezeMovementEffect), effectNamespace);
            Type movementEffectType = new Type(typeof(MovementEffect), effectNamespace);

            typeManager.RegisterType(iMovementType);
            typeManager.RegisterType(allyMovementType);
            typeManager.RegisterType(enemyMovementType);
            typeManager.RegisterType(playerMovementType);
            typeManager.RegisterType(freezeMovementEffectType);
            typeManager.RegisterType(movementEffectType);
            #endregion
        }
        #endregion
    }
}