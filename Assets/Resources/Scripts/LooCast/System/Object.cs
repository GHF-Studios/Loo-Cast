

using LooCast.System.Identification;

namespace LooCast.System
{
    public abstract class Object : IObject, IInstanceRegistryProvider
    {
        public Registry<InstanceIdentifier, CSharpInstance> LooCastInstances => throw new global::System.NotImplementedException();
    }
}
