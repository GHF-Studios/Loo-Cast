namespace LooCast.System.Factories
{
    using global::LooCast.System.MetaData;

    public class ComponentFactory : Factory<Component, ComponentMetaData>
    {
        public override Component CreateInstance(ComponentMetaData metadata)
        {
            return new Component(metadata);
        }
    }
}