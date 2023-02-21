using System;
using UnityEngine;

namespace LooCast.Projectile
{
    using LooCast.System;
    using LooCast.System.Management;

    public class ProjectileManager : ModuleManager
    {
        #region Static Properties
        public static ProjectileManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[ProjectileManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<ProjectileManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static ProjectileManager instance;
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
            looCastNamespace = new Namespace("Projectile", rootNamespace);
            looCastType = new Type(typeof(ProjectileManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type projectileType = new Type(typeof(Projectile), looCastNamespace);
            Type chargedPlasmaProjectileType = new Type(typeof(ChargedPlasmaProjectile), looCastNamespace);
            Type laserProjectileType = new Type(typeof(LaserProjectile), looCastNamespace);
            Type multiplexerProjectileType = new Type(typeof(MultiplexerProjectile), looCastNamespace);
            Type multiplexerFragmentProjectileType = new Type(typeof(MultiplexerFragmentProjectile), looCastNamespace);

            typeManager.RegisterType(projectileType);
            typeManager.RegisterType(chargedPlasmaProjectileType);
            typeManager.RegisterType(laserProjectileType);
            typeManager.RegisterType(multiplexerProjectileType);
            typeManager.RegisterType(multiplexerFragmentProjectileType);
            #endregion
        }
        #endregion
    }
}