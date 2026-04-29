using System;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;

    public sealed class Component : IIdentifiable, IDisposable
    {
        #region Properties
        public Identifier Identifier => componentIdentifier;
        public ComponentIdentifier ComponentIdentifier => componentIdentifier;

        public Guid ComponentInstanceGUID => componentInstanceGUID;
        public UnityEngine.Component ComponentInstance => componentInstance;

        public Type ComponentType => componentType;
        public GameObject ContainingGameObject => containingGameObject;
        #endregion

        #region Fields
        private ComponentIdentifier componentIdentifier;

        private Guid componentInstanceGUID;
        private UnityEngine.Component componentInstance;

        private Type componentType;
        private GameObject containingGameObject;

        private bool disposed = false;
        #endregion

        #region Constructors
#nullable enable 
        public Component(Type componentType, GameObject containingGameObject)
        {
            if (!componentType.CSSystemType.IsSubclassOf(typeof(UnityEngine.MonoBehaviour)))
            {
                throw new ArgumentException("The componentType must be of Type UnityEngine.MonoBehaviour");
            }

            this.componentType = componentType;
            this.containingGameObject = containingGameObject;

            componentIdentifier = new ComponentIdentifier(containingGameObject.GameObjectIdentifier, componentType.TypeIdentifier, Guid.NewGuid());
            componentInstanceGUID = componentIdentifier.ComponentInstanceGUID;
            componentInstance = containingGameObject.GameObjectInstance.AddComponent(componentType.CSSystemType);

            containingGameObject.ContainedComponents.Add(this);
        }
#nullable disable
        #endregion

        #region Finalizer
        ~Component()
        {
            Dispose(false);
        }
        #endregion

        #region Methods
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        private void Dispose(bool disposing)
        {
            if (!disposed)
            {
                if (disposing)
                {
                    // Dispose managed resources here, if any.
                    containingGameObject.ContainedComponents.Remove(this);
                    UnityEngine.Object.Destroy(componentInstance);
                    componentInstance = null;
                }

                // Dispose unmanaged resources here, if any.
                disposed = true;
            }
        }

        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is Component otherComponent)
            {
                return Equals(otherComponent);
            }
            return false;
        }

        public bool Equals(Component otherComponent)
        {
            return ComponentIdentifier.Equals(otherComponent.ComponentIdentifier);
        }

        public override int GetHashCode()
        {
            return ComponentIdentifier.GetHashCode();
        }

        public override string ToString()
        {
            return ComponentIdentifier.ToString();
        }
        #endregion
    }
}
