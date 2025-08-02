//
// SqratTypes: Type Translators
//

//
// Copyright (c) 2009 Brandon Jones
//
// This software is provided 'as-is', without any express or implied
// warranty. In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
//  1. The origin of this software must not be misrepresented; you must not
//  claim that you wrote the original software. If you use this software
//  in a product, an acknowledgment in the product documentation would be
//  appreciated but is not required.
//
//  2. Altered source versions must be plainly marked as such, and must not be
//  misrepresented as being the original software.
//
//  3. This notice may not be removed or altered from any source
//  distribution.
//

#if !defined(_SCRAT_TYPES_H_)
#define _SCRAT_TYPES_H_

#include <squirrel.h>
#include <string>

#include "sqratClassType.h"

namespace Sqrat {

	// copied from http://www.experts-exchange.com/Programming/Languages/CPP/A_223-Determing-if-a-C-type-is-convertable-to-another-at-compile-time.html
	template <typename T1, typename T2>
	struct is_convertible
	{
	private:
		struct True_ { char x[2]; };
		struct False_ { };

		static True_ helper(T2 const &);
		static False_ helper(...);

		static T1* dummy;

	public:
		static bool const YES = (
			sizeof(True_) == sizeof(is_convertible::helper(*dummy))
			);
	};

	template <typename T, bool b>
	struct popAsInt
	{
		T value;
		popAsInt(HSQUIRRELVM vm, SQInteger idx)
		{
			SQObjectType value_type = sq_gettype(vm, idx);
			switch (value_type) {
			case OT_BOOL:
				SQBool sqValueb;
				sq_getbool(vm, idx, &sqValueb);
				value = static_cast<T>(sqValueb);
				break;
			case OT_INTEGER:
				SQInteger sqValue;
				sq_getinteger(vm, idx, &sqValue);
				value = static_cast<T>(sqValue);
				break;
			case OT_FLOAT:
				SQFloat sqValuef;
				sq_getfloat(vm, idx, &sqValuef);
				value = static_cast<T>(sqValuef);
				break;
			default:
				value = static_cast<T>(0);
				break;
			}
		}
	};

	template <typename T>
	struct popAsInt<T, false>
	{
		T value;  // cannot be initialized because unknown constructor parameters
		popAsInt(HSQUIRRELVM /*vm*/, SQInteger /*idx*/)
		{
			// keep the current error message already set previously, do not touch that here
		}
	};

	template <typename T>
	struct popAsFloat
	{
		T value;
		popAsFloat(HSQUIRRELVM vm, SQInteger idx)
		{
			SQObjectType value_type = sq_gettype(vm, idx);
			switch (value_type) {
			case OT_BOOL:
				SQBool sqValueb;
				sq_getbool(vm, idx, &sqValueb);
				value = static_cast<T>(sqValueb);
				break;
			case OT_INTEGER:
				SQInteger sqValue; \
					sq_getinteger(vm, idx, &sqValue);
				value = static_cast<T>(sqValue);
				break;
			case OT_FLOAT:
				SQFloat sqValuef;
				sq_getfloat(vm, idx, &sqValuef);
				value = static_cast<T>(sqValuef);
				break;
			default:
				value = 0;
				break;
			}
		}
	};

//
// Variable Accessors
//

// Generic classes
template<class T>
struct Var {
    T value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        value = *ClassType<T>::GetInstance(vm, idx);
    }
    static void push(HSQUIRRELVM vm, T value) {
        ClassType<T>::PushInstanceCopy(vm, value);
    }
};

template<class T>
struct Var<T&> {
    T value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        value = *ClassType<T>::GetInstance(vm, idx);
    }
    static void push(HSQUIRRELVM vm, T value) {
        ClassType<T>::PushInstanceCopy(vm, value);
    }
};

template<class T>
struct Var<T*> {
    T* value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        value = ClassType<T>::GetInstance(vm, idx);
    }
    static void push(HSQUIRRELVM vm, T* value) {
		if( value == NULL )
			sq_pushnull( vm );
		else
			ClassType<T>::PushInstance(vm, value);
    }
};

template<class T>
struct Var<const T&> {
    T value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        value = *ClassType<T>::GetInstance(vm, idx);
    }
    static void push(HSQUIRRELVM vm, T value) {
        ClassType<T>::PushInstanceCopy(vm, value);
    }
};

template<class T>
struct Var<const T*> {
    T* value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        value = ClassType<T>::GetInstance(vm, idx);
    }
    static void push(HSQUIRRELVM vm, T* value) {
        ClassType<T>::PushInstance(vm, value);
    }
};

// Integer Types
#define SCRAT_INTEGER( type ) \
 template<> \
 struct Var<type> { \
     type value; \
     Var(HSQUIRRELVM vm, SQInteger idx) { \
         value = popAsInt<type, true>(vm, idx).value; \
     } \
     static void push(HSQUIRRELVM vm, type& value) { \
         sq_pushinteger(vm, static_cast<SQInteger>(value)); \
     } \
 };\
 \
 template<> \
 struct Var<const type> { \
     type value; \
     Var(HSQUIRRELVM vm, SQInteger idx) { \
         value = popAsInt<type, true>(vm, idx).value; \
     } \
     static void push(HSQUIRRELVM vm, const type& value) { \
         sq_pushinteger(vm, static_cast<SQInteger>(value)); \
     } \
 }; \
 \
 template<> \
 struct Var<const type&> { \
     type value; \
     Var(HSQUIRRELVM vm, SQInteger idx) { \
		value = popAsInt<type, true>(vm, idx).value; \
     } \
     static void push(HSQUIRRELVM vm, const type& value) { \
         sq_pushinteger(vm, static_cast<SQInteger>(value)); \
     } \
 };

SCRAT_INTEGER(unsigned int)
SCRAT_INTEGER(signed int)
SCRAT_INTEGER(unsigned long)
SCRAT_INTEGER(signed long)
SCRAT_INTEGER(unsigned short)
SCRAT_INTEGER(signed short)

#if defined(__int64)
SCRAT_INTEGER(unsigned __int64)
SCRAT_INTEGER(signed __int64)
#endif

// Float Types
#define SCRAT_FLOAT( type ) \
 template<> \
 struct Var<type> { \
     type value; \
     Var(HSQUIRRELVM vm, SQInteger idx) { \
		value = popAsFloat<type>(vm, idx).value; \
     } \
     static void push(HSQUIRRELVM vm, type& value) { \
         sq_pushfloat(vm, static_cast<SQFloat>(value)); \
     } \
 }; \
 \
 template<> \
 struct Var<const type> { \
     type value; \
     Var(HSQUIRRELVM vm, SQInteger idx) { \
		value = popAsFloat<type>(vm, idx).value; \
     } \
     static void push(HSQUIRRELVM vm, const type& value) { \
         sq_pushfloat(vm, static_cast<SQFloat>(value)); \
     } \
 }; \
 template<> \
 struct Var<const type&> { \
     type value; \
     Var(HSQUIRRELVM vm, SQInteger idx) { \
		value = popAsFloat<type>(vm, idx).value; \
     } \
     static void push(HSQUIRRELVM vm, const type& value) { \
         sq_pushfloat(vm, static_cast<SQFloat>(value)); \
     } \
 };

SCRAT_FLOAT(float)
SCRAT_FLOAT(double)

// Boolean Types
template<>
struct Var<bool> {
    bool value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        SQBool sqValue;
        sq_tobool(vm, idx, &sqValue);
        value = (sqValue != 0);
    }
    static void push(HSQUIRRELVM vm, bool& value) {
        sq_pushbool(vm, static_cast<SQBool>(value));
    }
};

template<>
struct Var<const bool> {
    bool value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        SQBool sqValue;
        sq_tobool(vm, idx, &sqValue);
        value = (sqValue != 0);
    }
    static void push(HSQUIRRELVM vm, const bool& value) {
        sq_pushbool(vm, static_cast<SQBool>(value));
    }
};

template<>
struct Var<const bool&> {
    bool value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        SQBool sqValue;
        sq_tobool(vm, idx, &sqValue);
        value = (sqValue != 0);
    }
    static void push(HSQUIRRELVM vm, const bool& value) {
        sq_pushbool(vm, static_cast<SQBool>(value));
    }
};

// String Types
typedef std::basic_string<SQChar> string;

template<>
struct Var<SQChar*> {
    SQChar* value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        sq_tostring(vm, idx);
        sq_getstring(vm, -1, (const SQChar**)&value);
        sq_pop(vm,1);
    }
    static void push(HSQUIRRELVM vm, SQChar* value) {
        sq_pushstring(vm, value, -1);
    }
};

template<>
struct Var<const SQChar*> {
    const SQChar* value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        sq_tostring(vm, idx);
        sq_getstring(vm, -1, &value);
        sq_pop(vm,1);
    }
    static void push(HSQUIRRELVM vm, const SQChar* value) {
        sq_pushstring(vm, value, -1);
    }
};

template<>
struct Var<string> {
    string value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        const SQChar* ret;
        sq_tostring(vm, idx);
        sq_getstring(vm, -1, &ret);
        value = string(ret);
        sq_pop(vm,1);
    }
    static void push(HSQUIRRELVM vm, string value) {
        sq_pushstring(vm, value.c_str(), -1);
    }
};

template<>
struct Var<string&> {
    string value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        const SQChar* ret;
        sq_tostring(vm, idx);
        sq_getstring(vm, -1, &ret);
        value = string(ret);
        sq_pop(vm,1);
    }
    static void push(HSQUIRRELVM vm, string value) {
        sq_pushstring(vm, value.c_str(), -1);
    }
};

template<>
struct Var<const string&> {
    string value;
    Var(HSQUIRRELVM vm, SQInteger idx) {
        const SQChar* ret;
        sq_tostring(vm, idx);
        sq_getstring(vm, -1, &ret);
        value = string(ret);
        sq_pop(vm,1);
    }
    static void push(HSQUIRRELVM vm, string value) {
        sq_pushstring(vm, value.c_str(), -1);
    }
};

//
// Variable Accessors
//

// Push
template<class T>
inline void PushVar(HSQUIRRELVM vm, T value) {
    Var<T>::push(vm, value);
}
}

#endif
