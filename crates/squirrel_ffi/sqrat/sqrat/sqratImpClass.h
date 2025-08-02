namespace Sqrat
{
    template<class C>
    class ImprovedAllocator :public DefaultAllocator<C>
	{
        static SQInteger setInstance(HSQUIRRELVM vm,C* instance)
        {
            sq_setinstanceup(vm, 1, instance);
            sq_setreleasehook(vm, 1, &DefaultAllocator<C>::Delete);
            return 0;
        }
    public:
        template <typename A1>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value
                ));
        }
        template <typename A1,typename A2>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value,
                Var<A2>(vm, 3).value
                ));
        }
        template <typename A1,typename A2,typename A3>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value,
                Var<A2>(vm, 3).value,
                Var<A3>(vm, 4).value
                ));
        }
        template <typename A1,typename A2,typename A3,typename A4>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value,
                Var<A2>(vm, 3).value,
                Var<A3>(vm, 4).value,
                Var<A4>(vm, 5).value
                ));
        }
        template <typename A1,typename A2,typename A3,typename A4,typename A5>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value,
                Var<A2>(vm, 3).value,
                Var<A3>(vm, 4).value,
                Var<A4>(vm, 5).value,
                Var<A5>(vm, 6).value
                ));
        }
        template <typename A1,typename A2,typename A3,typename A4,typename A5,typename A6>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value,
                Var<A2>(vm, 3).value,
                Var<A3>(vm, 4).value,
                Var<A4>(vm, 5).value,
                Var<A5>(vm, 6).value,
                Var<A6>(vm, 7).value
                ));
        }
        template <typename A1,typename A2,typename A3,typename A4,typename A5,typename A6,typename A7>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value,
                Var<A2>(vm, 3).value,
                Var<A3>(vm, 4).value,
                Var<A4>(vm, 5).value,
                Var<A5>(vm, 6).value,
                Var<A6>(vm, 7).value,
                Var<A7>(vm, 8).value
                ));
        }
        template <typename A1,typename A2,typename A3,typename A4,typename A5,typename A6,typename A7,typename A8>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value,
                Var<A2>(vm, 3).value,
                Var<A3>(vm, 4).value,
                Var<A4>(vm, 5).value,
                Var<A5>(vm, 6).value,
                Var<A6>(vm, 7).value,
                Var<A7>(vm, 8).value,
                Var<A8>(vm, 9).value
                ));
        }
        template <typename A1,typename A2,typename A3,typename A4,typename A5,typename A6,typename A7,typename A8,typename A9>
        static SQInteger iNew(HSQUIRRELVM vm) {
            return setInstance(vm, new C(
                Var<A1>(vm, 2).value,
                Var<A2>(vm, 3).value,
                Var<A3>(vm, 4).value,
                Var<A4>(vm, 5).value,
                Var<A5>(vm, 6).value,
                Var<A6>(vm, 7).value,
                Var<A7>(vm, 8).value,
                Var<A8>(vm, 9).value,
                Var<A9>(vm, 10).value
                ));
        }
    };


    template<class C,  class A = ImprovedAllocator<C> >
    class ImprovedClass :public Class<C,A>
	{
        ImprovedClass& BindConstructor(SQFUNCTION method,SQInteger nParams){
            HSQUIRRELVM vm = Object::GetVM();
            HSQOBJECT& classObj = ClassType<C>::ClassObject(vm);
            sq_pushobject(vm, classObj);
            sq_pushstring(vm,_SC("constructor"), -1);
            sq_newclosure(vm, method, 0);
            sq_setparamscheck(vm,nParams+1,NULL);
            sq_newslot(vm, -3, false);
            sq_pop(vm, 1);
            return *this;
        }
    public:
        // Create a new table
        ImprovedClass(HSQUIRRELVM v = DefaultVM::Get(), bool createClass = true) : Class<C,A>(v,createClass) {}
        template<class A1>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1>,1);
        }
        template<class A1,class A2>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1,A2>,2);
        }
        template<class A1,class A2,class A3>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1,A2,A3>,3);
        }
        template<class A1,class A2,class A3,class A4>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1,A2,A3,A4>,4);
        }
        template<class A1,class A2,class A3,class A4,class A5>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1,A2,A3,A4,A5>,5);
        }
        template<class A1,class A2,class A3,class A4,class A5,class A6>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1,A2,A3,A4,A5,A6>,6);
        }
        template<class A1,class A2,class A3,class A4,class A5,class A6,class A7>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1,A2,A3,A4,A5,A6,A7>,7);
        }
        template<class A1,class A2,class A3,class A4,class A5,class A6,class A7,class A8>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1,A2,A3,A4,A5,A6,A7,A8>,8);
        }
        template<class A1,class A2,class A3,class A4,class A5,class A6,class A7,class A8,class A9>
        ImprovedClass& Ctor() {
            return BindConstructor(A::template iNew<A1,A2,A3,A4,A5,A6,A7,A8,A9>,9);
        }
    };
}