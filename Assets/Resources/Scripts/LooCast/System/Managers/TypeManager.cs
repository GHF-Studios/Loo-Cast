using System;
using UnityEngine;

namespace LooCast.System.Managers
{
    using global::LooCast.System.Exceptions;
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Registries;

    public sealed class TypeManager : InternalManager
    {
        #region Static Properties
        public static TypeManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new TypeManager();
                    instance.UnityEngineGameObject.name = "[TypeManager]";
                    instance.UnityEngineGameObject.layer = 31;
                    instance.UnityEngineGameObject.tag = "INTERNAL";
                    instance.UnityEngineGameObject.transform.parent = LooCast.Instance.gameObject.transform;
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static TypeManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private TypeRegistry typeRegistry;
        #endregion

        #region Constructors
        public TypeManager() : base("LooCast.System.Managers:TypeManager", MainManager.Instance)
        {
            typeRegistry = new TypeRegistry();
        }
        #endregion

        #region Methods
        public void RegisterType(Type type)
        {
            TypeIdentifier typeIdentifier = type.TypeIdentifier;
            if (typeRegistry.ContainsKey(typeIdentifier))
            {
                throw new Exception($"[TypeManager] Type '{typeIdentifier}' already exists!");
            }

            typeRegistry.Add(type.TypeIdentifier, type);
        }

        public Type GetType(TypeIdentifier typeIdentifier)
        {
            if (!typeRegistry.TryGetValue(typeIdentifier, out Type type))
            {
                throw new Exception($"[TypeManager] Type '{typeIdentifier}' could not be found!");
            }

            return type;
        }

        public Type GetType<T>()
        {
            // TODO: Implement
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();
        }

        public override void InitializeInstance()
        {
            base.InitializeInstance();
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();
        }
        #endregion
    }
}