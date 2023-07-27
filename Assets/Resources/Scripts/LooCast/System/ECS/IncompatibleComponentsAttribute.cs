using System;

namespace LooCast.System.ECS
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public sealed class IncompatibleComponentsAttribute : Attribute
    {
        #region Properties
        public Type[] IncompatibleComponentTypes { get; private set; }
        #endregion

        #region Constructors
        public IncompatibleComponentsAttribute(params Type[] incompatibleComponentTypes)
        {
            foreach (Type incompatibleComponentType in incompatibleComponentTypes)
            {
                if (!typeof(Component).IsAssignableFrom(incompatibleComponentType))
                {
                    throw new ArgumentException($"The provided type '{incompatibleComponentType.Name}' is not a component type!");
                }
            }
            IncompatibleComponentTypes = incompatibleComponentTypes;
        }
        #endregion
    }
}
